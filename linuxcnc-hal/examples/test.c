
//
// Copyright (C) 2009 scotta at CNCZone.com, alan_3301 at CNCZone.com
//     2014 Benjamin Brockhaus
//     2015 Sebastian Kuzminsky <seb@highlab.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301 USA
//


#ifndef ULAPI
#error This is intended as a userspace component only.
#endif

#include <stdio.h>
#include <unistd.h>
#include <time.h>
#include <string.h>
#include <stdlib.h>
#include <signal.h>
#include <errno.h>
#include <getopt.h>
#include <math.h>
#include "rtapi.h"
#include "hal.h"


#define HYCOMM_MIN_OK       10


// bits in Status Data
#define STATUS_SetF         0x00
#define STATUS_OutF         0x01
#define STATUS_OutA         0x02
#define STATUS_RoTT         0x03
#define STATUS_DCV          0x04
#define STATUS_ACV          0x05
#define STATUS_Cont         0x06
#define STATUS_Tmp          0x07

// control commands CNTR
#define CONTROL_Run_Fwd     0x01
#define CONTROL_Run_Rev     0x11
#define CONTROL_Stop        0x08

// control responses CNST
#define CONTROL_Run         0x01
#define CONTROL_Jog         0x02
#define CONTROL_Command_rf  0x04
#define CONTROL_Running     0x08
#define CONTROL_Jogging     0x10
#define CONTROL_Running_rf  0x20
#define CONTROL_Bracking    0x40
#define CONTROL_Track_Start 0x80



int hycomm_ok;                      // set hycomm-ok bit if last HYCOMM_OK transactions went well
int debug;
int slave = 1;

/* HAL data struct */
typedef struct {
    // The HAL comp name will be set to <string>, and all pin and parameter
    // names will begin with <string>.

    hal_bit_t   *enable;                // bit to enable this component

    hal_float_t *Set_F;                 // frequency command
    hal_float_t *Out_F;                 // actual output frequency
    hal_float_t *Out_A;                 // actual output amps
    hal_float_t *RoTT;                  // actual motor rmp (based on VFD parameters)
    hal_float_t *DCV;                   // DC Volts (to be confirmed)
    hal_float_t *ACV;                   // AC Volts (to be confirmed)
    hal_float_t *Cont;
    hal_float_t *Tmp;                   // Temperature (to be confirmed)

    hal_bit_t *spindle_forward;         // spindle forward input
    hal_bit_t *spindle_reverse;         // spindle reverse input
    hal_bit_t *spindle_on;              // spinlde on input
    hal_float_t *CNTR;                  // stores the status of the control request
    hal_float_t *CNST;                  // stores the response of the control request

    hal_bit_t *CNST_Run;                // CNST Run bit
    hal_bit_t *CNST_Jog;                // CNST Jog bit
    hal_bit_t *CNST_Command_rf;         // CNST Run reverse / forward bit
    hal_bit_t *CNST_Running;            // CNST Running bit
    hal_bit_t *CNST_Jogging;            // CNST Jogging bit
    hal_bit_t *CNST_Running_rf;             // CNST Jog reverse / forward bit
    hal_bit_t *CNST_Bracking;           // CNST bracking bit
    hal_bit_t *CNST_Track_Start;        // CNST track start bit

    hal_float_t *speed_command;         // spindle speed command from EMC
    hal_float_t *freq_cmd;              // calculated frequency command

    hal_float_t *max_freq;              // PD005 Max Operating Frequency
    hal_float_t *base_freq;             // PD004 Base Frequency
    hal_float_t *freq_lower_limit;      // PD011 Frequency Lower Limit
    hal_float_t *rated_motor_voltage;   // PD141 Rated Motor Voltage - as per motor name plate
    hal_float_t *rated_motor_current;   // PD142 Rated Motor Current - as per motor name plate
    hal_u32_t *motor_poles;                 // PD143 Number of motor poles - from motor name plate
    hal_float_t *rated_motor_rev;       // max motor speed (at max_freq).  PD144 gets set to value corresponding to RPM at 50Hz

    hal_bit_t   *hycomm_ok;             // the last HYCOMM_OK transactions returned successfully

    hal_float_t *max_rpm;               // calculated based on VFD max frequency setup parameter

        hal_float_t     *spindle_speed_fb;   // (out) reports current spindle speed
        hal_bit_t       *spindle_at_speed;   // (out) True when spindle is on and at commanded speed
        hal_float_t     *spindle_at_speed_tolerance;  // (in)

    hal_float_t retval;
    hal_s32_t   errorcount;
    hal_float_t looptime;
    //hal_float_t   motor_nameplate_hz;     // speeds are scaled in Hz, not RPM
    //hal_float_t   motor_nameplate_RPM;    // nameplate RPM at default Hz
    //hal_float_t   rpm_limit;              // do-not-exceed output frequency
    //hal_bit_t *acc_dec_pattern;       // if set: choose ramp times as defined in F500+F501
                                        // if zero (default): choose ramp times as defined in ACC and DEC
    //hal_float_t   upper_limit_hz;         // VFD setup parameter - maximum output frequency in HZ

    //hal_bit_t old_run;                // so we can detect changes in the run state
    //hal_bit_t old_dir;
    //hal_bit_t old_err_reset;
    //hal_u32_t old_cmd1_reg;               // copy of last write to FA00
    //hal_u32_t failed_reg;             // remember register for failed hycomm transaction - aids debugging
} haldata_t;

static int done;
static void quit_signal_handler(int sig) {
    done = 1;
}

char *modname = "test";

static struct option long_options[] = {
        {"bits", 1, 0, 'b'},
        {"device", 1, 0, 'd'},
        {"debug", 0, 0, 'g'},
        {"help", 0, 0, 'h'},
        {"name", 1, 0, 'n'},
        {"parity", 1, 0, 'p'},
        {"rate", 1, 0, 'r'},
        {"stopbits", 1, 0, 's'},
        {"target", 1, 0, 't'},
                {"max-frequency", 1, 0, 'F'},
                {"min-frequency", 1, 0, 'f'},
                {"base-frequency", 1, 0, 'B'},
                {"motor-voltage", 1, 0, 'V'},
                {"motor-current", 1, 0, 'I'},
                {"motor-speed", 1, 0, 'S'},
                {"motor-poles", 1, 0, 'P'},
        {0,0,0,0}
};

static char *option_string = "b:d:ghn:p:r:s:t:F:f:B:V:I:S:P:";

static char *bitstrings[] = {"5", "6", "7", "8", NULL};
static char *paritystrings[] = {"even", "odd", "none", NULL};
static char *ratestrings[] = {"110", "300", "600", "1200", "2400", "4800", "9600",
        "19200", "38400", "57600", "115200", NULL};
static char *stopstrings[] = {"1", "2", NULL};

int match_string(char *string, char **matches) {
    int len, which, match;
    which=0;
    match=-1;
    if ((matches==NULL) || (string==NULL)) return -1;
    len = strlen(string);
    while (matches[which] != NULL) {
        if ((!strncmp(string, matches[which], len)) && (len <= strlen(matches[which]))) {
            if (match>=0) return -1;        // multiple matches
            match=which;
        }
        ++which;
    }
    return match;
}

void usage(int argc, char **argv) {
    printf("Usage:  %s [options]\n", argv[0]);
    printf(
            "This is a userspace HAL program, typically loaded using the halcmd \"loadusr\" command:\n"
            "    loadusr hy_vfd\n"
            "There are several command-line options.  Options that have a set list of possible values may\n"
            "    be set by using any number of characters that are unique.  For example, --rate 5 will use\n"
            "    a baud rate of 57600, since no other available baud rates start with \"5\"\n"
            "-b or --bits <n> (default 8)\n"
            "    Set number of data bits to <n>, where n must be from 5 to 8 inclusive\n"
            "-d or --device <path> (default /dev/ttyS0)\n"
            "    Set the name of the serial device node to use\n"
            "-g or --debug\n"
            "    Turn on debugging messages.  Debug mode will cause all hycomm messages\n"
                        "    to be printed in hex on the terminal.\n"
            "-n or --name <string> (default hy_vfd)\n"
            "    Set the name of the HAL module.  The HAL comp name will be set to <string>, and all pin\n"
            "    and parameter names will begin with <string>.\n"
            "-p or --parity {even,odd,none} (default even)\n"
            "    Set serial parity to even, odd, or none.\n"
            "-r or --rate <n> (default 19200)\n"
            "    Set baud rate to <n>.  It is an error if the rate is not one of the following:\n"
            "    110, 300, 600, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200\n"
            "-s or --stopbits {1,2} (default 1)\n"
            "    Set serial stop bits to 1 or 2\n"
            "-t or --target <n> (default 1)\n"
            "    Set HYCOMM target (slave) number.  This must match the device number you set on the Huanyang VFD.\n"
                        "-F or --max-frequency <f>\n"
                        "    Set VFD max frequency to <f> Hz.  This will be read from the VFD\n"
                        "    register 5 if not supplied on the command line.\n"
                        "-f or --min-frequency <f>\n"
                        "    Set VFD min frequency to <f> Hz.  This will be read from the VFD\n"
                        "    register 11 if not supplied on the command line.\n"
                        "-B or --base-frequency <f>\n"
                        "    Set VFD base frequency to <f> Hz.  This will be read from the VFD\n"
                        "    register 11 if not supplied on the command line.\n"
                        "-V or --motor-voltage <v>\n"
                        "    Set VFD max output voltage to <v> (Volts).  This will be read from the\n"
                        "    VFD register 141 if not supplied on the command line.\n"
                        "-I or --motor-current <v>\n"
                        "    Set VFD max output current to <i> (Amps).  This will be read from the\n"
                        "    VFD register 142 if not supplied on the command line.\n"
                        "-S or --motor-speed <r>\n"
                        "    Set the motor's max speed to <r> (RPM).  This will be computed from the\n"
                        "    50 Hz RPM value read from the VFD register P144 if not supplied on the\n"
                        "    command line.\n"
                        "-P or --motor-poles <n>\n"
                        "    Set VFD register PD143 (Number of Motor Poles) to <n>.  This value\n"
                        "    comes from the motor's name plate.\n"
        );
}



int main(int argc, char **argv)
{
    int retval;
    haldata_t *haldata;
    int hal_comp_id;
    struct timespec loop_timespec, remaining;
    int baud, bits, stopbits;
    char *device, *parity, *endarg;
    int opt;
    int argindex, argvalue;

        float max_freq = 0;
        float base_freq = 0;
        float min_freq = 0;
        float motor_v = 0;
        float motor_i = 0;
        float motor_speed = 0;
        hal_u32_t motor_poles = 0;

    done = 0;

        // assume that nothing is specified on the command line
    baud = 19200;
    bits = 8;
    stopbits = 1;
    device = "/dev/ttyS0";
    parity = "even";
    slave = 1;

    // Booshit
    printf("TESTO: Process options\n");

    // process command line options
    while ((opt=getopt_long(argc, argv, option_string, long_options, NULL)) != -1) {
        switch(opt) {
        case 'b':   // serial data bits, probably should be 8 (and defaults to 8)
            argindex=match_string(optarg, bitstrings);
            if (argindex<0) {
                printf("hy_vfd: ERROR: invalid number of bits: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            bits = atoi(bitstrings[argindex]);
            break;
        case 'd':   // device name, default /dev/ttyS0
            // could check the device name here, but we'll leave it to the library open
            if (strlen(optarg) > FILENAME_MAX) {
                printf("hy_vfd: ERROR: device node name is too long: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            device = strdup(optarg);
            break;
        case 'g':
            debug = 1;
            break;
        case 'n':   // module base name
            if (strlen(optarg) > HAL_NAME_LEN-20) {
                printf("hy_vfd: ERROR: HAL module name too long: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            modname = strdup(optarg);
            break;
        case 'p':   // parity, should be a string like "even", "odd", or "none"
            argindex=match_string(optarg, paritystrings);
            if (argindex<0) {
                printf("hy_vfd: ERROR: invalid parity: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            parity = paritystrings[argindex];
            break;
        case 'r':   // Baud rate, 19200 default
            argindex=match_string(optarg, ratestrings);
            if (argindex<0) {
                printf("hy_vfd: ERROR: invalid baud rate: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            baud = atoi(ratestrings[argindex]);
            break;
        case 's':   // stop bits, defaults to 1
            argindex=match_string(optarg, stopstrings);
            if (argindex<0) {
                printf("hy_vfd: ERROR: invalid number of stop bits: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            stopbits = atoi(stopstrings[argindex]);
            break;
        case 't':   // target number (HYCOMM ID), default 1
            argvalue = strtol(optarg, &endarg, 10);
            if ((*endarg != '\0') || (argvalue < 1) || (argvalue > 254)) {
                printf("hy_vfd: ERROR: invalid slave number: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            slave = argvalue;
            break;

                case 'F':
                        max_freq = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (max_freq < 0.0) || (max_freq > UINT16_MAX)) {
                printf("hy_vfd: ERROR: invalid max frequency: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'B':
                        base_freq = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (base_freq < 0.0) || (base_freq > 400.0)) {
                printf("hy_vfd: ERROR: invalid base frequency: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'f':
                        min_freq = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (min_freq < 0.0) || (min_freq > UINT16_MAX)) {
                printf("hy_vfd: ERROR: invalid min frequency: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'V':
                        motor_v = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (motor_v < 0.0) || (motor_v > UINT16_MAX)) {
                printf("hy_vfd: ERROR: invalid motor max voltage: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'I':
                        motor_i = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (motor_i < 0.0) || (motor_i > UINT16_MAX)) {
                printf("hy_vfd: ERROR: invalid motor max current: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'S':
                        motor_speed = strtof(optarg, &endarg);
            if ((*endarg != '\0') || (motor_speed < 0.0) || (motor_speed > UINT16_MAX)) {
                printf("hy_vfd: ERROR: invalid motor max speed: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

                case 'P':
                        motor_poles = strtof(optarg, &endarg);
            if (*endarg != '\0') {
                printf("hy_vfd: ERROR: invalid motor poles: %s\n", optarg);
                retval = -1;
                goto out_noclose;
            }
            break;

        case 'h':
        default:
            usage(argc, argv);
            exit(0);
            break;
        }
    }

    if (debug) {
        printf("%s: device='%s', baud=%d, bits=%d, parity='%s', stopbits=%d, address=%d, debug=%d, PID=%d\n",
                modname,device, baud, bits, parity, stopbits, slave, debug, getpid());
    }

    // Booshit
    printf("TESTO: Signal handlers\n");

        /* SIGTERM and SIGINT trigger a clean shutdown */
        signal(SIGINT, quit_signal_handler);
        signal(SIGTERM, quit_signal_handler);

    /* Assume 19.2k E-8-1 serial settings, device 1 */
    // hycomm_init(&hc_param, device, baud, parity, bits, stopbits);
    // hc_param.print_errors = 1;

    // /* the open has got to work, or we're out of business */
    // if (((retval = hycomm_connect(&hc_param))!=0) || done) {
    //     printf("%s: ERROR: couldn't open serial device\n", modname);
    //     goto out_noclose;
    // }


    /* create HAL component */
    hal_comp_id = hal_init(modname);

    // Booshit
    printf("TESTO: Initialised\n");

    if ((hal_comp_id < 0) || done) {
        printf("%s: ERROR: hal_init failed\n", modname);
        retval = hal_comp_id;
        goto out_close;
    }

    /* grab some shmem to store the HAL data in */
    haldata = (haldata_t *)hal_malloc(sizeof(haldata_t));

    if ((haldata == 0) || done) {
        printf("%s: ERROR: unable to allocate shared memory\n", modname);
        retval = -1;
        goto out_close;
    }

    // Booshit
    printf("TESTO: Malloc\n");

    retval = hal_pin_bit_newf(HAL_IN, &(haldata->enable), hal_comp_id, "%s.enable", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_float_newf(HAL_OUT, &(haldata->Set_F), hal_comp_id, "%s.SetF", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->Out_F), hal_comp_id, "%s.OutF", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->Out_A), hal_comp_id, "%s.OutA", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->RoTT), hal_comp_id, "%s.Rott", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->DCV), hal_comp_id, "%s.DCV", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->ACV), hal_comp_id, "%s.ACV", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->Cont), hal_comp_id, "%s.Cont", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->Tmp), hal_comp_id, "%s.Tmp", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_bit_newf(HAL_IN, &(haldata->spindle_forward), hal_comp_id, "%s.spindle-forward", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_IN, &(haldata->spindle_reverse), hal_comp_id, "%s.spindle-reverse", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_IN, &(haldata->spindle_on), hal_comp_id, "%s.spindle-on", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->CNTR), hal_comp_id, "%s.CNTR", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->CNST), hal_comp_id, "%s.CNST", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Run), hal_comp_id, "%s.CNST-run", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Jog), hal_comp_id, "%s.CNST-jog", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Command_rf), hal_comp_id, "%s.CNST-command-rf", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Running), hal_comp_id, "%s.CNST-running", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Jogging), hal_comp_id, "%s.CNST-jogging", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Running_rf), hal_comp_id, "%s.CNST-running-rf", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Bracking), hal_comp_id, "%s.CNST-bracking", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->CNST_Track_Start), hal_comp_id, "%s.CNST-track-start", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_float_newf(HAL_IN, &(haldata->speed_command), hal_comp_id, "%s.speed-command", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->freq_cmd), hal_comp_id, "%s.frequency-command", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_float_newf(HAL_OUT, &(haldata->max_freq), hal_comp_id, "%s.max-freq", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->base_freq), hal_comp_id, "%s.base-freq", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->freq_lower_limit), hal_comp_id, "%s.freq-lower-limit", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->rated_motor_voltage), hal_comp_id, "%s.rated-motor-voltage", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->rated_motor_current), hal_comp_id, "%s.rated-motor-current", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_float_newf(HAL_OUT, &(haldata->rated_motor_rev), hal_comp_id, "%s.rated-motor-rev", modname);
    if (retval!=0) goto out_closeHAL;
    retval = hal_pin_u32_newf(HAL_OUT, &(haldata->motor_poles), hal_comp_id, "%s.motor-poles", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->hycomm_ok), hal_comp_id, "%s.hycomm-ok", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_param_s32_newf(HAL_RW, &(haldata->errorcount), hal_comp_id, "%s.error-count", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_param_float_newf(HAL_RW, &(haldata->retval), hal_comp_id, "%s.retval", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_float_newf(HAL_OUT, &(haldata->spindle_speed_fb), hal_comp_id, "%s.spindle-speed-fb", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_bit_newf(HAL_OUT, &(haldata->spindle_at_speed), hal_comp_id, "%s.spindle-at-speed", modname);
    if (retval!=0) goto out_closeHAL;

    retval = hal_pin_float_newf(HAL_IN, &(haldata->spindle_at_speed_tolerance), hal_comp_id, "%s.spindle-at-speed-tolerance", modname);
    if (retval!=0) goto out_closeHAL;

    // Booshit
    printf("TESTO: Pins\n");

    /* make default data match what we expect to use */

    *(haldata->enable) = 0;

    *(haldata->Set_F) = 0;
    *(haldata->Out_F) = 0;
    *(haldata->Out_A) = 0;
    *(haldata->RoTT) = 0;
    *(haldata->DCV) = 0;
    *(haldata->ACV) = 0;
    *(haldata->Cont) = 0;
    *(haldata->Tmp) = 0;

    *(haldata->spindle_forward) = 0;
    *(haldata->spindle_reverse) = 0;
    *(haldata->spindle_on) = 0;
    *(haldata->freq_cmd) = 0;
    *(haldata->CNTR) = 0;
    *(haldata->CNST) = 0;

    *(haldata->max_freq) = max_freq;
    *(haldata->base_freq) = base_freq;
    *(haldata->freq_lower_limit) = min_freq;
    *(haldata->rated_motor_voltage) = motor_v;
    *(haldata->rated_motor_current) = motor_i;
    *(haldata->rated_motor_rev) = motor_speed;
    *(haldata->motor_poles) = motor_poles;

    *(haldata->hycomm_ok) = 0;

    *haldata->spindle_speed_fb = 0.0;
    *haldata->spindle_at_speed = 0;
    *haldata->spindle_at_speed_tolerance = 0.02;

    // hc_data.slave = slave;
    haldata->errorcount = 0;
    haldata->looptime = 0.1;


    //haldata->motor_nameplate_hz = 50; // folks in The Colonies typically would use 60Hz and 1730 rpm
    //haldata->motor_nameplate_RPM = 1410;
    //haldata->rpm_limit = MAX_RPM;
    //haldata->acc_dec_pattern = 0;

    //haldata->old_run = -1;        // make sure the initial value gets output
    //haldata->old_dir = -1;
    //haldata->old_err_reset = -1;
    //haldata->failed_reg = 0;

    hal_ready(hal_comp_id);
    // hc_data.slave = slave;

    // Booshit
    printf("TESTO: Waiting for enable %i\n", hal_comp_id);

    // // wait until EMC and AXIS is ready, ie enable bit is set
    // while (!*(haldata->enable)){
    //     // do nothing until enabled
    //             usleep(10*1000);
    //             if (done) {
    //                 // killed by signal
    //                 exit(0);
    //             }
    // }

    // Booshit
    printf("TESTO: Enabled\n");

    // // read the VFD setup parameters
    //     do {
    //         retval = read_setup(&hc_param, &hc_data, haldata);
    //         if (retval != 0) {
    //             fprintf(stderr, "error reading setup from VFD, retrying\n");
    //             usleep(100 * 1000);
    //         }
    //         if (done) {
    //             // killed by signal
    //             exit(0);
    //         }
    //     } while (retval != 0);

    // here's the meat of the program.  loop until done (which may be never)
    while (done==0) {

        // Booshit
        printf("TESTO: %f %p\n", *(haldata->speed_command), haldata->speed_command);

        // if (*(haldata->enable)) {
        //     // Read inputs
        //     if (read_data(&hc_param, &hc_data, haldata) < 0) {
        //         hycomm_ok = 0;
        //     } else {
        //         hycomm_ok++;
        //     }

        //     // Set outputs
        //     if (write_data(&hc_param, &hc_data, haldata) < 0) {
        //         hycomm_ok = 0;
        //     } else {
        //         hycomm_ok++;
        //     }
        // }



        if (hycomm_ok > HYCOMM_MIN_OK) {
            *(haldata->hycomm_ok) = 1;
        } else {
            *(haldata->hycomm_ok) = 0;
        }


        // don't want to scan too fast, and shouldn't delay more than a few seconds
        if (haldata->looptime < 0.001) haldata->looptime = 0.001;
        if (haldata->looptime > 1.0) haldata->looptime = 1.0;
        loop_timespec.tv_sec = (time_t)(haldata->looptime);
        loop_timespec.tv_nsec = (long)((haldata->looptime - loop_timespec.tv_sec) * 1000000000l);
        nanosleep(&loop_timespec, &remaining);

    }

    retval = 0; /* if we get here, then everything is fine, so just clean up and exit */

    out_closeHAL:
    hal_exit(hal_comp_id);
    out_close:
    // hycomm_close(&hc_param);
    out_noclose:
    return retval;
}

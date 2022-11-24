# procps

Bindings for `libprocps` v3.3 with structured output.

Fully tested by parsing output from the [procps-ng](https://gitlab.com/procps-ng/procps/-/tree/v3.3.17) command-line utilities.

## Current Implementation
- [ ] sysinfo.h
  - [x] uptime
  - [ ] btime
  - [x] loadavg 
  - [ ] meminfo
    - [x] the important stuff
    - [ ] others
  - [ ] getstat
  - [ ] partitions
  - [ ] diskstat
  - [ ] slabinfo
  - [ ] pid_digits
  - [ ] cpuinfo
- [x] version.h

Header file support is incremental, once one is done development on the next begins.
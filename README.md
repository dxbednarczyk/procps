# procps

Bindings for `libprocps` v3.3 with structured output.

Tested by parsing output from the [procps-ng](https://gitlab.com/procps-ng/procps/-/tree/v3.3.17) command-line utilities.

## Current Implementation
- [ ] sysinfo.h
  - [x] uptime
  - [x] btime
  - [x] loadavg 
  - [ ] meminfo
    - [x] the important stuff
    - [ ] others
  - [ ] vminfo
  - [x] getstat
  - [x] diskstat
  - [ ] slabinfo
  - [ ] pid_digits
  - [ ] cpuinfo
- [x] version.h
- [x] whattime.h
- [x] wchan.h

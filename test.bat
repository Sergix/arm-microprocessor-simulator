@echo off

set tests1="btest" "ctest"
set tests2="branch" "quicksort_no_io" "cmp" "locals_no_io" "mersenne_no_io"

(for %%a in (%tests1%) do ( 
    @REM .\src-tauri\target\debug\armsim.exe --traceall --exec ..\cps310-class_files\tests\sim1\\%%a.exe
    .\src-tauri\target\release\armsim.exe --traceall --exec ..\cps310-class_files\tests\sim1\\%%a.exe
    rename trace.log %%a_trace.log
    move %%a_trace.log .\tests\logs
    echo %%a trace log generated
)
)

(for %%a in (%tests2%) do ( 
    @REM .\src-tauri\target\debug\armsim.exe --traceall --exec ..\cps310-class_files\tests\sim2\\%%a.exe
    .\src-tauri\target\release\armsim.exe --traceall --exec ..\cps310-class_files\tests\sim2\\%%a.exe
    rename trace.log %%a_trace.log
    move %%a_trace.log .\tests\logs
    echo %%a trace log generated
)
)
@echo off

IF "%1"=="" (
    echo missing input file name
    goto end
)

set input_filepath=%1


FOR %%i in ("%input_filepath%") DO set filename=%%~ni%%~xi

IF not exist %input_filepath% (
    echo The file %filename% doesn't exist
    goto end
)


cmd /c wasm-opt -Oz %input_filepath% --output temp-%filename%

wasm-snip temp-%filename% --output temp2-%filename% --snip-rust-fmt-code --snip-rust-panicking-code

cmd /c wasm-opt --dce temp2-%filename% --output optimized-%filename%

del temp-%filename%

del temp2-%filename%

FOR %%A IN ("%input_filepath%") DO set in_size=%%~zA
FOR %%A IN ("optimized-%filename%") DO set out_size=%%~zA

echo %in_size% bytes -^> %out_size% bytes, see optimized-%filename%

:end
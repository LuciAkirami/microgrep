# microgrep

microgrep tool allows you to search for the lines containing a desired word from a given file.

## Example with cargo run, in Powershell/Command Prompt:

### Without Case Sensitive Search
    
    $Env:IGNORE_CASE=1; cargo run -- to poem.txt 
  -> will search for "to" in "poem.txt"

### With Case Sensitive Search
    
    Remove-Item Env:IGNORE_CASE   
    cargo run to poem.txt 
  -> will search for "to" in "poem.txt"

## Example with binary, in Powershell/Command Prompt:

### Without Case Sensitive Search

    $Env:IGNORE_CASE=1; 
    .\target\debug\microgrep.exe to poem.txt 
  -> will search for "to" in "poem.txt"

### With Case Sensitive Search

    Remove-Item Env:IGNORE_CASE
    .\target\debug\microgrep.exe to poem.txt 
  -> will search for "to" in "poem.txt"

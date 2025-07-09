# Install instructions
## Prerequisite
Ensure you have the correct rules in main.rs before you compile and install it.

## Install the program

Ensure that you are at the root of the project then install the program by
running: `cargo install --path .`

Install a new version by doing the same comand when you update it.



## Install by making sure these keys exits in regidit.exe.
Please don't use the installer script. It is not working for now.

```reg
HKEY_CLASSES_ROOT
   Redirectify
      (Default) = "Redirectify URL Handler"
      FriendlyTypeName = "Redirectify"
      DefaultIcon
         (Default) = "%USERPROFILE%\.cargo\bin\redirectify.exe",0
      shell
         open
            command
               (Default) = "%USERPROFILE%\.cargo\bin\redirectify.exe" "%1"
```




## Install browser extensions
The browser extension is for being able to Ctrl + Shift + Left Click then it will
then try to open the link in your preferred browser.


You would have to install either FireMonkey (Firefox) or Violentmonkey
for (chrome derivates).
After you have installed the FireMonkey or Violentmonkey you should be able to
just press the link down below and it will ask you to install it.
[Open with redirectify ](.\open_with_redirectify.user.js)


## Install by making sure these keys exits.
Please don't use the installer script. it is not working for now

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

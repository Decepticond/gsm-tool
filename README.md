## GSM-TOOL 
*Created by Aleksandr S*

Howdy and thanks for checking out `gsm-tool`. It's a set of commands made in rust aimed to aid furthered ethical education of GSM packet decoding, and cryptography in general.
This project is currently still very WIP, as is my knowledge of GitHub in general :skull:.


***This project has since been retired, all commands are being moved over crypt-tool***

### Installation [WIP] 
**To install on MacOS/Linux, simply run:**

`curl -L https://raw.githubusercontent.com/Lazaurus/gsm-tool/main/install.sh | bash`

*Or if you would prefer another path*

`curl -L https://raw.githubusercontent.com/Lazaurus/gsm-tool/main/install.sh | bash -s mypath/`
* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

**To install on Windows, [run as administrator], simply run:**

`Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

*then run...*

`Start-BitsTransfer -Source https://raw.githubusercontent.com/Lazaurus/gsm-tool/main/install.ps1 -Destination $env:TMP/install_gsm-tool.ps1; Unblock-File $env:TMP/install_gsm-tool.ps1; Invoke-Expression $env:TMP/install_gsm-tool.ps1`


_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ 

Congrats, gsm-tool should've been succesfully installed, run gsm-tool to start it up.


### Uninstalling gsm-tool
`rm -rf gsm-tool`
...
`cargo uninstall gsm-tool`



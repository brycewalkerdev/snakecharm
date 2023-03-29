# Things to remember

pyinstaller [filename] --onefile --add-data 'resources/FILENAME.txt'
https://pyinstaller.org/en/stable/spec-files.html#adding-data-files

https://stackoverflow.com/questions/7674790/bundling-data-files-with-pyinstaller-onefile/13790741#13790741

https://stackoverflow.com/questions/53587322/how-do-i-include-files-with-pyinstaller

## Dev
pyinstaller --clean --onefile -y -n "backend" --add-data="resources\pp.txt;resources" backend.py


## Build
pyinstaller --clean --onefile -y -n "backend-x86_64-pc-windows-msvc" --add-data="resources\pp.txt;resources" backend.py

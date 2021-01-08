```
docker build -t  ubuntu:sqlite -f  Dockerfile .
docker run -it --rm ubuntu:sqlite /bin/bash
git clone https://github.com/sqlite/sqlite.git
cd sqlite
./configure
make test
make quicktest
make tcltest
```

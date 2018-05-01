windows with mingw64

run 
make_winlib

linux
g++ stargo_ftbl.cpp -fPIC -shared -o ../libstargo_getftbl.so  -I../srplab/stargo/include -DENV_LINUX
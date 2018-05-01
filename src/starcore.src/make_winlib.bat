ar -x ./lib.windows/libstarlib.a
gcc -c starrust_ftbl.cpp -I./include -fPIC
gcc -c vsopenapi_c_stub.c -I./include -fPIC
gcc -c starrust_native.c -I./include -fPIC
ar rcs ../windows.static/libvsopenapi_c_stub.a starrust_ftbl.o vsopenapi_c_stub.o starrust_native.o vs_shell.o vs_uuid.o srplib_main.o md5proc.o md5proc_c.o
del *.o


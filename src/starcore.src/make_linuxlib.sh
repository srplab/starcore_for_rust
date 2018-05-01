ar -x ./lib.linux/libstarlib.a
gcc -c starrust_ftbl.cpp -I./include -fPIC -DENV_LINUX -DENV_M64
gcc -c vsopenapi_c_stub.c -I./include -fPIC -DENV_LINUX -DENV_M64
gcc -c starrust_native.c -I./include -fPIC -DENV_LINUX -DENV_M64
ar rcs ../linux.static/libvsopenapi_c_stub.a starrust_ftbl.o vsopenapi_c_stub.o starrust_native.o vs_shell.o vs_uuid.o srplib_main.o md5proc.o md5proc_c.o
rm *.o


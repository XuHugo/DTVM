#!/bin/bash
#rm -rf ./libzetaengine.a ./libutils_lib.a
#cp -f ../bazel-bin/libzetaengine.a .
#cp -f ../bazel-bin/libutils_lib.a .
#cp -f ../bazel-bin/external/zen_deps_asmjit/libasmjit.a .
# cp -f ../bazel-bin/external/+third_party_deps+zen_deps_asmjit/libasmjit.a .

system_name="$(uname -s)"

export CPP_COMPILER=g++

if [ "$system_name" = "Darwin" ]; then
  # if on mac, need brew install gcc@11
  GCC11_PATH=`brew --prefix gcc@11`
  echo "gcc@11 path:"
  echo $GCC11_PATH
  export PATH=$GCC11_PATH/bin:$PATH
  export CPP_COMPILER=g++-11
fi

LIBSTDCPP_PATH=`$CPP_COMPILER -print-file-name=libstdc++.a`
cp -f $LIBSTDCPP_PATH .

# make sure the files has enough permission to be used in rust
chmod 755 libstdc++.a
chmod 755 libzetaengine.a
chmod 755 libutils_lib.a
chmod 755 libasmjit.a

if [ "$system_name" = "Darwin" ]; then
  # rust on mac need link libgcc_s lib
  cp $GCC11_PATH/lib/gcc/11/libgcc_s.1.1.dylib .
chmod 755 libgcc_s.1.1.dylib
fi

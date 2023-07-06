fn main() {
        cc::Build::new()
            .object("src/addxx.o")
  //          .object("/home/user/risc_0/simple-project/lib/lib_a-malloc.o")
  //                  .object("src/randomx_united.o")
  //                  .object("/opt/riscv/riscv32-unknown-elf/lib/libc.a")
  //                  .cpp_link_stdlib("/opt/riscv/riscv32-unknown-elf/lib/libstdc++.a")
                    .compile("addxx");
}

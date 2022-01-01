((c-mode . ((helm-make-build-dir . "build")
	    (lsp-clients-clangd-args . ("--query-driver=arm-unknown-eabi-gcc"
										"--compile-commands-dir=projecttest/igloo/target/samd21j18a"
                                        "--pch-storage=memory"
                                        "--background-index"
                                        "-j=4"
                                        ))
            )))

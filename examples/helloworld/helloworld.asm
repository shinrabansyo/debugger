$helloworld
    string "Hello world!"

===

@func_main
    // フレームポインタの設定
    addi r2 = r0, 0x100

    // "Hello world!"
    addi r10 = r0, $helloworld
    beq r1, (r0, r0) -> @func_print

    // 無限ループ
    @loop.func_main
        beq r0, (r0, r0) -> @loop.func_main

@func_print
    // フレームポインタの退避
    subi r2 = r2, 4
    sw r2[0] = r3
    addi r3 = r2, 0

    // レジスタの退避
    subi r2 = r2, 4
    sw r3[-4] = r1

    // 文字列出力ループ
    add r4 = r0, r10
    @loop.func_print
        // 文字列取得
        lb r5 = r4[0]

        // NULLチェック
        beq r0, (r5, r0) -> @end.loop.func_print

        // 出力
        out r0[0] = r5

        // カウンタ加算
        addi r4 = r4, 1
        beq r0, (r0, r0) -> @loop.func_print
    @end.loop.func_print

    // 改行文字出力
    addi r4 = r0, 10
    out r0[0] = r4

    // レジスタの復元
    lw r1 = r3[-4]
    addi r2 = r2, 4

    // フレームポインタの復元
    lw r3 = r3[0]
    addi r2 = r2, 4

    // return
    jal r0, r1[0]

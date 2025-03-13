$fizz
    string "fizz"
$buzz
    string "buzz"
$fizzbuzz
    string "fizzbuzz"

===

@func_main
    // フレームポインタの設定
    addi r2 = r0, 0x100

    // FizzBuzz ループ
    addi r20 = r0, 0
    @loop.func_main
        // ループカウンタ加算
        add r4 = r0, r20
        addi r4 = r4, 1
        add r20 = r0, r4

        // 終了判定
        addi r5 = r0, 100
        ble r0, (r5, r20) -> @inf_loop.func_main

        // 分岐 (÷15)
        add r10 = r0, r20
        addi r5 = r0, 15
        add r11 = r0, r5
        beq r1, (r0, r0) -> @func_div
        beq r0, (r11, r0) -> @fifteen.loop.func_main

        // 分岐 (÷3)
        add r10 = r0, r20
        addi r5 = r0, 3
        add r11 = r0, r5
        beq r1, (r0, r0) -> @func_div
        beq r0, (r11, r0) -> @three.loop.func_main

        // 分岐 (÷5)
        add r10 = r0, r20
        addi r5 = r0, 5
        add r11 = r0, r5
        beq r1, (r0, r0) -> @func_div
        beq r0, (r11, r0) -> @five.loop.func_main

        // 分岐不成立なら数字を出力して，ループの先頭へ
        add r10 = r0, r20
        beq r1, (r0, r0) -> @func_print_num
        beq r0, (r0, r0) -> @loop.func_main

        // "Fizz"
        @three.loop.func_main
            addi r10 = r0, $fizz
            beq r1, (r0, r0) -> @func_print_str
            beq r0, (r0, r0) -> @loop.func_main

        // "Buzz"
        @five.loop.func_main
            addi r10 = r0, $buzz
            beq r1, (r0, r0) -> @func_print_str
            beq r0, (r0, r0) -> @loop.func_main

        // "Buzz"
        @fifteen.loop.func_main
            addi r10 = r0, $fizzbuzz
            beq r1, (r0, r0) -> @func_print_str
            beq r0, (r0, r0) -> @loop.func_main

    // 無限ループ
    @inf_loop.func_main
        beq r0, (r0, r0) -> @inf_loop.func_main

@func_print_num
    // フレームポインタの退避
    subi r2 = r2, 4
    sw r2[0] = r3
    addi r3 = r2, 0

    // レジスタの退避
    subi r2 = r2, 4
    sw r3[-4] = r1

    // 数値出力
    beq r1, (r0, r0) -> @func_print_num_rec

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

@func_print_num_rec
    // フレームポインタの退避
    subi r2 = r2, 4
    sw r2[0] = r3
    addi r3 = r2, 0

    // レジスタの退避
    subi r2 = r2, 12
    sw r3[-4] = r1
    sw r3[-8] = r20
    sw r3[-12] = r21

    // 出力対象の保存
    add r20 = r0, r10

    // 最上位桁取得
    add r10 = r0, r20
    addi r5 = r0, 10
    add r11 = r0, r5
    beq r1, (r0, r0) -> @func_div

    // 余りを保存
    add r21 = r0, r11

    // 商が 0 以上なら再帰
    blt r1, (r0, r10) -> @func_print_num_rec

    // 余りを復元
    add r5 = r0, r21

    // ASCII 文字変換
    addi r5 = r5, 48

    // 出力
    out r0[0] = r5

    // レジスタの復元
    lw r1 = r3[-4]
    lw r20 = r3[-8]
    lw r21 = r3[-12]
    addi r2 = r2, 12

    // フレームポインタの復元
    lw r3 = r3[0]
    addi r2 = r2, 4

    // return
    jal r0, r1[0]

@func_print_str
    // フレームポインタの退避
    subi r2 = r2, 4
    sw r2[0] = r3
    addi r3 = r2, 0

    // レジスタの退避
    subi r2 = r2, 4
    sw r3[-4] = r1

    // 文字列出力ループ
    add r4 = r0, r10
    @loop.func_print_str
        // 文字列取得
        lb r5 = r4[0]

        // NULLチェック
        beq r0, (r5, r0) -> @end.loop.func_print_str

        // 出力
        out r0[0] = r5

        // カウンタ加算
        addi r4 = r4, 1
        beq r0, (r0, r0) -> @loop.func_print_str
    @end.loop.func_print_str

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

@func_div
    // フレームポインタの退避
    subi r2 = r2, 4
    sw r2[0] = r3
    addi r3 = r2, 0

    // レジスタの退避
    subi r2 = r2, 4
    sw r3[-4] = r1

    // 引き算ループ
    add r4 = r0, r10
    add r5 = r0, r11
    addi r6 = r0, 0
    @loop.func_div
        // 終了判定
        blt r0, (r4, r5) -> @end.loop.func_div

        // 引き算
        sub r4 = r4, r5
        addi r6 = r6, 1
        beq r0, (r0, r0) -> @loop.func_div
    @end.loop.func_div

    // 結果の格納
    add r10 = r0, r6
    add r11 = r0, r4

    // レジスタの復元
    lw r1 = r3[-4]
    addi r2 = r2, 4

    // フレームポインタの復元
    lw r3 = r3[0]
    addi r2 = r2, 4

    // return
    jal r0, r1[0]

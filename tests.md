# Test Results

There are several test ROMs designed to test the edge cases of various operations.

Note that even many widely used emulators don't pass all the tests, but using the tests is still a good way to track my progress and potential issues.

## Legend

:white_check_mark: - Test passes

:x: - Test fails

:warning: - Functionality unimplemented

:question: - Test is unclear

## Blargg Tests

| Test number | cgb_sound | cpu_instrs         | dmg_sound | instr_timing | interrupt_time | mem_timing | mem_timing-2 | oam_bug            |
| ----------- | --------- | ----------         | --------- | ------------ | -------------- | ---------- | ------------ | -------            |
| 1           | :warning: | :white_check_mark: | :warning: | :x:          | :x:            | :x:        | :x:          | :x:                |
| 2           | :warning: | :white_check_mark: | :warning: |              |                | :x:        | :x:          | :x:                |
| 3           | :warning: | :white_check_mark: | :warning: |              |                | :x:        | :x:          | :white_check_mark: |
| 4           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 5           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 6           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :white_check_mark: |
| 7           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 8           | :warning: | :white_check_mark: | :warning: |              |                |            |              | :x:                |
| 9           | :warning: | :white_check_mark: | :warning: |              |                |            |              |                    |
| 10          | :warning: | :white_check_mark: | :warning: |              |                |            |              |                    |
| 11          | :warning: | :white_check_mark: | :warning: |              |                |            |              |                    |
| 12          | :warning: |                    | :warning: |              |                |            |              |                    |

## Mooneye Tests

### Acceptance

| Test                      | Success            |
| ------------------------- | ------------------ |
| add_sp_e_timing           | :x:                |
| boot_div2-S               | :x:                |
| boot_div-dmg0             | :x:                |
| boot_div-dmgABCmgb        | :x:                |
| boot_div-S                | :x:                |
| boot_hwio-dmg0            | :x:                |
| boot_hwio-dmgABCmgb       | :x:                |
| boot_hwio-S               | :x:                |
| boot_regs-dmg0            | :x:                |
| boot_regs-dmgABC          | :white_check_mark: |
| boot_regs-mgb             | :x:                |
| boot_regs-sgb2            | :x:                |
| boot_regs-sgb             | :x:                |
| call_cc_timing2           | :x:                |
| call_cc_timing            | :x:                |
| call_timing2              | :x:                |
| call_timing               | :x:                |
| di_timing-GS              | :x:                |
| div_timing                | :x:                |
| ei_sequence               | :x:                |
| ei_timing                 | :x:                |
| halt_ime0_ei              | :white_check_mark: |
| halt_ime0_nointr_timing   | :x:                |
| halt_ime1_timing2-GS      | :x:                |
| halt_ime1_timing          | :white_check_mark: |
| if_ie_registers           | :x:                |
| intr_timing               | :x:                |
| jp_cc_timing              | :x:                |
| jp_timing                 | :x:                |
| ld_hl_sp_e_timing         | :x:                |
| oam_dma_restart           | :x:                |
| oam_dma_start             | :warning:          |
| oam_dma_timing            | :x:                |
| pop_timing                | :x:                |
| push_timing               | :x:                |
| rapid_di_ei               | :x:                |
| ret_cc_timing             | :x:                |
| reti_intr_timing          | :x:                |
| reti_timing               | :x:                |
| ret_timing                | :x:                |
| rst_timing                | :x:                |

#### Bits

| Test              | Success            |
| ----------------- | ------------------ |
| mem_oam           | :white_check_mark: |
| reg_f             | :white_check_mark: |
| unused_hwio       | :question:         |

#### Instr

| Test              | Success            |
| ----------------- | ------------------ |
| daa               | :white_check_mark: |

#### Interrupts

| Test              | Success            |
| ----------------- | ------------------ |
| ie_push           | :question:         |

#### OAM DMA

| Test              | Success            |
| ----------------- | ------------------ |
| basic             | :white_check_mark: |
| reg_read          | :x:                |
| sources-GS        | :x:                |

#### PPU

| Test                            | Success            |
| ------------------------------- | ------------------ |
| hblank_ly_scx_timing-GS         | :x:                |
| intr_1_2_timing-GS              | :x:                |
| intr_2_0_timing                 | :x:                |
| intr_2_mode0_timing_sprites     | :x:                |
| intr_2_mode0_timing             | :x:                |
| intr_2_mode3_timing             | :x:                |
| intr_2_oam_ok_timing            | :x:                |
| lcdon_timing-GS                 | :x:                |
| lcdon_write_timing-GS           | :x:                |
| stat_irq_blocking               | :x:                |
| stat_lyc_onoff                  | :x:                |
| vblank_stat_intr-GS             | :x:                |

#### Serial

| Test                                | Success            |
| ----------------------------------- | ------------------ |
| boot_sclk_align-dmgABCmgb           | :x:                |

#### Timer

| Test                      | Success            |
| ------------------------- | ------------------ |
| div_write                 | :x:                |
| rapid_toggle              | :x:                |
| tim00_div_trigger         | :x:                |
| tim00                     | :x:                |
| tim01_div_trigger         | :x:                |
| tim01                     | :x:                |
| tim10_div_trigger         | :x:                |
| tim10                     | :x:                |
| tim11_div_trigger         | :x:                |
| tim11                     | :x:                |
| tima_reload               | :x:                |
| tima_write_reloading      | :x:                |
| tma_write_reloading       | :x:                |

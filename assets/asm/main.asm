.n64
.relativeinclude on

; prevent people from building with older armips versions
.if (version() < 110)
.notice version()
.error "Detected armips build is too old. Please install https://github.com/Kingcom/armips version 0.11 or later."
.endif

.create "../generated/asm-patched.n64", 0
.incbin "../base.n64"

;==================================================================================================
; Base game editing region
;==================================================================================================

.include "hacks.asm"

;==================================================================================================
; New code region
;==================================================================================================

.headersize (0x80400000 - 0x03480000)

.org    0x80400000
.area   0x00200000 ; payload max memory
PAYLOAD_START:

.area 0x20, 0
RANDO_CONTEXT:
.endarea

PAYLOAD_END:
.endarea ; payload max memory

.close

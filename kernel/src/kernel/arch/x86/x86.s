.intel_syntax noprefix

.global reloadSegments

reloadSegments:
   mov   ax, 0x10
   mov   ds, ax
   mov   es, ax
   mov   fs, ax
   mov   gs, ax
   mov   ss, ax
   call get_retaddr_ppro
   add eax, 7 # seven bytes between this and .reload_cs
   pushd 0x8 # kernel code segment
   push eax # address calculated earlier
   retf # RETurn Far to .reload_cs despite no call earlier

.reload_cs:
   ret

get_retaddr_ppro: # returns the return address via eax
   mov  eax, [esp]
   ret
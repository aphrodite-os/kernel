.intel_syntax noprefix

.global reloadSegments
.global get_addr_actual

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

# get the address of something. ebx contains the supposed address of this function; ecx contains the supposed address of the target.
# Output is in ecx. Clobbers eax, ebx.
get_addr_actual:
   call get_retaddr_ppro
   sub eax, 5
   sub ebx, eax
   add ecx, ebx
   ret

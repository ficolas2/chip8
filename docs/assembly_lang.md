The program is loaded into memory at address 0x200, and the program counter is set to 0x200.
The program counter is incremented by 2 after each instruction.

## Instructions
| Assembly       | Instruction | Description                                            |
| -------------- | ----------- | ------------------------------------------------------ |
| cls            | 00E0        | Clear the screen                                       |
| rts            | 00EE        | Return from subroutine call.                           |
| jmp nn         | 01nn        | Jump to addr.                                          |
| jsr nnn        | 2nnn        | Jump to subroutine at addr nnn. Stack size is 16.      |
| jmi nnn        | Bnnn        | Jump to addr nnn + register v0.                        |
| skeq vX nn     | 3xnn        | Skip if register vX = nn.                              |
| skeq vX vY     | 5xy0        | Skip if registers vX = vY.                             |
| skne vX nn     | 4xnn        | Skip if register vX ≠ nn.                              |
| skne vX vY     | 9xy0        | Skip if registers vX ≠ vY.                             |
| mov vX nn      | 6xnn        | Move nn to register vX.                                |
| mov vX vY      | 8xy0        | Move register vY to vX.                                |
| add vX nn      | 7xnn        | Add nn to register vX. No carry generated.             |
| add vX vY      | 8xy4        | Add register vY to vX. Carry in register vF.           |
| sub vX vY      | 8xy5        | Subtract register vY from vX. Borrow from register vF. |
| rsb vX vY      | 8xy7        | Subtract register vX from vY. Borrow from register vF. |
| or vX vY       | 8xy1        | Bitwise OR register vY to vX.                          |
| and vX vY      | 8xy2        | Bitwise AND register vY to vX.                         |
| xor vX vY      | 8xy3        | Bitwise XOR register vY to vX.                         |
| shl vX         | 8xyE        | Shift register vX left by 1. Bit 7 in register vF.     |
| shr vX         | 8xy6        | Shift register vX right by 1. Bit 0 in register vF.    |
| mvi nnn        | Annn        | Move register I to nnn.                                |        
| rand vX nn     | Cxnn        | Generate random number less than or equal to nn        |
| skpr vX        | Ex9E        | Skip if key in register vX is pressed.                 |
| skup vX        | ExA1        | Skip if key in register vX is not pressed.             |
| key vX         | Fx0A        | Wait until key in register vX is pressed               |
| gdelay vX      | Fx07        | Set register vX to the value of the delay timer.       |
| sdelay vX      | Fx15        | Set delay timer to the value in register vX            |
| ssound vX      | Fx18        | Set sound timer to the value in register vX            |
| adi vX         | Fx1E        | Add register vX to register I.                         |
| str vX         | Fx55        | Store registers v0 to vX in memory starting at I.      |
| ldr vX         | Fx65        | Load registers v0 to vX from memory starting at I.     |
| bcd vX         | Fx33        | Store BCD representation of register vX in memory.     |
| sprite vX vY n | Dxyn        | Draw sprite at vX, vY with height n.                   |
| font vX        | Fx29        | Set I to the location of the sprite for the character in register vX. |
| end           | 0000        | End of program. This is a custom instruction.           |

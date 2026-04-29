`cargo run -p evmdecode-cli -- decode --tx $TX --rpc $RPC_URL`

tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
from    : 0x946Aa581287709B59dB1e635DAF3c35408C20DEf
to      : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
value   : 0 wei
gas     : 84000
selector: 0xa9059cbb
signature: transfer(address,uint256)

Decoded Call
  selector : a9059cbb
  function : transfer(address,uint256)
+---------+-----------------------------------------------------+
| type    | value                                               |
+---------+-----------------------------------------------------+
| address | Address(0x4e5ae324d39935169cf35721b1fb31ed65d69974) |
+---------+-----------------------------------------------------+
| uint256 | Uint(187208650578, 256)                             |
+---------+-----------------------------------------------------+

`cargo run -p evmdecode-cli -- simulate --tx $TX --rpc $RPC_URL`
tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
from    : 0x946Aa581287709B59dB1e635DAF3c35408C20DEf
to      : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
block   : 24983425

Decoded Call
  selector : a9059cbb
  function : transfer(address,uint256)
+---------+-----------------------------------------------------+
| type    | value                                               |
+---------+-----------------------------------------------------+
| address | Address(0x4e5ae324d39935169cf35721b1fb31ed65d69974) |
+---------+-----------------------------------------------------+
| uint256 | Uint(187208650578, 256)                             |
+---------+-----------------------------------------------------+

Fetching state diff via debug_traceTransaction...
  warn: debug_ unavailable, showing receipt + logs

Receipt
  status   : success
  gas used : 40372

Event Logs
  [0] address : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
      event sig : 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
      topic[1]  : 0x000000000000000000000000946aa581287709b59db1e635daf3c35408c20def
      topic[2]  : 0x0000000000000000000000004e5ae324d39935169cf35721b1fb31ed65d69974
      data     : 0x0000000000000000000000000000000000000000000000000000002b96814f52


Gas Breakdown
  total      : 84000
  ├ intrinsic  : 21620 (25.7%)  [21000 base + calldata]
  ├ calldata   : 620 (0.7%)  [39 zero × 4 + 29 nonzero × 16]
  ├ logs/store : 2000 (2.4%)  [estimated]
  └ execution  : 60380 (71.9%)  [opcodes, memory]


`cargo run -p evmdecode-cli -- mev --tx $TX3 --rpc $RPC_URL`
tx : "0x3f56b8bf4752352d002295c06109b1d3a83fec5fc087fbf30a88979957f0db63"
from : "0x9ad2B40b5E0c42AF493BB1A4Ff9dBAA40C20A9E2"
block : 23618045

MEV Analysis
────────────────────────────────────────────────────────────
  ⟳ SWAP [92%]  █████████░
    Uniswap V3 swap across 2 pool(s)


`cargo run -p evmdecode-cli -- simulate --tx $TX --rpc $RPC_URL`
tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
from    : 0x946Aa581287709B59dB1e635DAF3c35408C20DEf
to      : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
block   : 24983425

Decoded Call
  selector : a9059cbb
  function : transfer(address,uint256)
+---------+-----------------------------------------------------+
| type    | value                                               |
+---------+-----------------------------------------------------+
| address | Address(0x4e5ae324d39935169cf35721b1fb31ed65d69974) |
+---------+-----------------------------------------------------+
| uint256 | Uint(187208650578, 256)                             |
+---------+-----------------------------------------------------+

Fetching state diff via debug_traceTransaction...
  warn: debug_ unavailable, showing receipt + logs

Receipt
  status   : success
  gas used : 40372

Event Logs
  [0] address : 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
      event sig : 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef
      topic[1]  : 0x000000000000000000000000946aa581287709b59db1e635daf3c35408c20def
      topic[2]  : 0x0000000000000000000000004e5ae324d39935169cf35721b1fb31ed65d69974
      data     : 0x0000000000000000000000000000000000000000000000000000002b96814f52


Gas Breakdown
  total      : 84000
  ├ intrinsic  : 21620 (25.7%)  [21000 base + calldata]
  ├ calldata   : 620 (0.7%)  [39 zero × 4 + 29 nonzero × 16]
  ├ logs/store : 2000 (2.4%)  [estimated]
  └ execution  : 60380 (71.9%)  [opcodes, memory]

  

`cargo run -p evmdecode-cli -- disasm --tx $TX --rpc $RPC_URL --functions`    
tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
to      : 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
block   : 0x17d3781
bytecode: 2186 bytes

Public functions (5)
────────────────────────────────────────────────────
SELECTOR      ENTRY       SIZE    
────────────────────────────────────────────────────
0x3659cfe6  0x0077      12 bytes
0x4f1ef286  0x00ba      76 bytes
0x5c60da1b  0x0108      12 bytes
0x8f283970  0x015f      12 bytes
0xf851a440  0x01a2      12 bytes

Internal functions / fallback (64)
────────────────────────────────────────────────────
TYPE          ENTRY       SIZE    
────────────────────────────────────────────────────
internal      0x006d      8 bytes
internal      0x0075      2 bytes
internal      0x0083      53 bytes
internal      0x00b8      2 bytes
internal      0x0106      2 bytes
internal      0x0114      9 bytes
internal      0x011d      66 bytes
internal      0x016b      53 bytes
internal      0x01a0      2 bytes
internal      0x01ae      9 bytes
internal      0x01b7      66 bytes
internal      0x01f9      8 bytes
internal      0x0201      11 bytes
internal      0x020c      5 bytes
internal      0x0211      2 bytes
internal      0x0213      8 bytes
internal      0x021b      60 bytes
internal      0x0257      5 bytes
internal      0x025c      8 bytes
internal      0x0264      1 bytes
internal      0x0265      3 bytes
internal      0x0268      8 bytes
internal      0x0270      60 bytes
internal      0x02ac      73 bytes
internal      0x02f5      5 bytes
internal      0x02fa      8 bytes
internal      0x0302      1 bytes
internal      0x0303      5 bytes
internal      0x0308      10 bytes
internal      0x0312      59 bytes
internal      0x034d      7 bytes
internal      0x0354      8 bytes
internal      0x035c      1 bytes
internal      0x035d      3 bytes
internal      0x0360      8 bytes
internal      0x0368      254 bytes
internal      0x0466      41 bytes
internal      0x048f      126 bytes
internal      0x050d      5 bytes
internal      0x0512      8 bytes
internal      0x051a      1 bytes
internal      0x051b      3 bytes
internal      0x051e      10 bytes
internal      0x0528      59 bytes
internal      0x0563      7 bytes
internal      0x056a      8 bytes
internal      0x0572      1 bytes
internal      0x0573      3 bytes
internal      0x0576      8 bytes
internal      0x057e      201 bytes
internal      0x0647      8 bytes
internal      0x064f      2 bytes
internal      0x0651      49 bytes
internal      0x0682      33 bytes
internal      0x06a3      5 bytes
internal      0x06a8      49 bytes
internal      0x06d9      9 bytes
internal      0x06e2      102 bytes
internal      0x0748      47 bytes
internal      0x0777      2 bytes
internal      0x0779      11 bytes
internal      0x0784      154 bytes
internal      0x081e      45 bytes
internal      0x084b      0 bytes

`cargo run -p evmdecode-cli -- disasm --tx $TX --rpc $RPC_URL`
tx      : 0x412f5f3c2f50993e7736150699c747ad5f682e20a610a0d1d5fa9d2fae466a4c
to      : 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48
block   : 0x17d3781
bytecode: 2186 bytes

OFFSET      OP    MNEMONIC        OPERAND
────────────────────────────────────────────────────────────
0x0000      60    PUSH1           0x80
0x0002      60    PUSH1           0x40
0x0004      52    MSTORE          
0x0005      60    PUSH1           0x04
0x0007      36    CALLDATASIZE    
0x0008      10    LT              
0x0009      61    PUSH2           0x006d
0x000c      57    JUMPI           
0x000d      60    PUSH1           0x00
0x000f      35    CALLDATALOAD    
0x0010      7c    PUSH29          0x0100000000000000000000000000000000000000000000000000000000
0x002e      90    SWAP1           
0x002f      04    DIV             
0x0030      63    PUSH4           0xffffffff
0x0035      16    AND             
0x0036      80    DUP1            
0x0037      63    PUSH4           0x3659cfe6
0x003c      14    EQ              
0x003d      61    PUSH2           0x0077
0x0040      57    JUMPI           
0x0041      80    DUP1            
0x0042      63    PUSH4           0x4f1ef286
0x0047      14    EQ              
0x0048      61    PUSH2           0x00ba
0x004b      57    JUMPI           
0x004c      80    DUP1            
0x004d      63    PUSH4           0x5c60da1b
0x0052      14    EQ              
0x0053      61    PUSH2           0x0108
0x0056      57    JUMPI           
0x0057      80    DUP1            
0x0058      63    PUSH4           0x8f283970
0x005d      14    EQ              
0x005e      61    PUSH2           0x015f
0x0061      57    JUMPI           
0x0062      80    DUP1            
0x0063      63    PUSH4           0xf851a440
0x0068      14    EQ              
0x0069      61    PUSH2           0x01a2
0x006c      57    JUMPI           
0x006d      5b    JUMPDEST        
0x006e      61    PUSH2           0x0075
0x0071      61    PUSH2           0x01f9
0x0074      56    JUMP            
0x0075      5b    JUMPDEST        
0x0076      00    STOP            
0x0077      5b    JUMPDEST        
0x0078      34    CALLVALUE       
0x0079      80    DUP1            
0x007a      15    ISZERO          
0x007b      61    PUSH2           0x0083
0x007e      57    JUMPI           
0x007f      60    PUSH1           0x00
0x0081      80    DUP1            
0x0082      fd    REVERT          
0x0083      5b    JUMPDEST        
0x0084      50    POP             
0x0085      61    PUSH2           0x00b8
0x0088      60    PUSH1           0x04
0x008a      80    DUP1            
0x008b      36    CALLDATASIZE    
0x008c      03    SUB             
0x008d      81    DUP2            
0x008e      01    ADD             
0x008f      90    SWAP1           
0x0090      80    DUP1            
0x0091      80    DUP1            
0x0092      35    CALLDATALOAD    
0x0093      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x00a8      16    AND             
0x00a9      90    SWAP1           
0x00aa      60    PUSH1           0x20
0x00ac      01    ADD             
0x00ad      90    SWAP1           
0x00ae      92    SWAP3           
0x00af      91    SWAP2           
0x00b0      90    SWAP1           
0x00b1      50    POP             
0x00b2      50    POP             
0x00b3      50    POP             
0x00b4      61    PUSH2           0x0213
0x00b7      56    JUMP            
0x00b8      5b    JUMPDEST        
0x00b9      00    STOP            
0x00ba      5b    JUMPDEST        
0x00bb      61    PUSH2           0x0106
0x00be      60    PUSH1           0x04
0x00c0      80    DUP1            
0x00c1      36    CALLDATASIZE    
0x00c2      03    SUB             
0x00c3      81    DUP2            
0x00c4      01    ADD             
0x00c5      90    SWAP1           
0x00c6      80    DUP1            
0x00c7      80    DUP1            
0x00c8      35    CALLDATALOAD    
0x00c9      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x00de      16    AND             
0x00df      90    SWAP1           
0x00e0      60    PUSH1           0x20
0x00e2      01    ADD             
0x00e3      90    SWAP1           
0x00e4      92    SWAP3           
0x00e5      91    SWAP2           
0x00e6      90    SWAP1           
0x00e7      80    DUP1            
0x00e8      35    CALLDATALOAD    
0x00e9      90    SWAP1           
0x00ea      60    PUSH1           0x20
0x00ec      01    ADD             
0x00ed      90    SWAP1           
0x00ee      82    DUP3            
0x00ef      01    ADD             
0x00f0      80    DUP1            
0x00f1      35    CALLDATALOAD    
0x00f2      90    SWAP1           
0x00f3      60    PUSH1           0x20
0x00f5      01    ADD             
0x00f6      91    SWAP2           
0x00f7      90    SWAP1           
0x00f8      91    SWAP2           
0x00f9      92    SWAP3           
0x00fa      93    SWAP4           
0x00fb      91    SWAP2           
0x00fc      92    SWAP3           
0x00fd      93    SWAP4           
0x00fe      90    SWAP1           
0x00ff      50    POP             
0x0100      50    POP             
0x0101      50    POP             
0x0102      61    PUSH2           0x0268
0x0105      56    JUMP            
0x0106      5b    JUMPDEST        
0x0107      00    STOP            
0x0108      5b    JUMPDEST        
0x0109      34    CALLVALUE       
0x010a      80    DUP1            
0x010b      15    ISZERO          
0x010c      61    PUSH2           0x0114
0x010f      57    JUMPI           
0x0110      60    PUSH1           0x00
0x0112      80    DUP1            
0x0113      fd    REVERT          
0x0114      5b    JUMPDEST        
0x0115      50    POP             
0x0116      61    PUSH2           0x011d
0x0119      61    PUSH2           0x0308
0x011c      56    JUMP            
0x011d      5b    JUMPDEST        
0x011e      60    PUSH1           0x40
0x0120      51    MLOAD           
0x0121      80    DUP1            
0x0122      82    DUP3            
0x0123      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0138      16    AND             
0x0139      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x014e      16    AND             
0x014f      81    DUP2            
0x0150      52    MSTORE          
0x0151      60    PUSH1           0x20
0x0153      01    ADD             
0x0154      91    SWAP2           
0x0155      50    POP             
0x0156      50    POP             
0x0157      60    PUSH1           0x40
0x0159      51    MLOAD           
0x015a      80    DUP1            
0x015b      91    SWAP2           
0x015c      03    SUB             
0x015d      90    SWAP1           
0x015e      f3    RETURN          
0x015f      5b    JUMPDEST        
0x0160      34    CALLVALUE       
0x0161      80    DUP1            
0x0162      15    ISZERO          
0x0163      61    PUSH2           0x016b
0x0166      57    JUMPI           
0x0167      60    PUSH1           0x00
0x0169      80    DUP1            
0x016a      fd    REVERT          
0x016b      5b    JUMPDEST        
0x016c      50    POP             
0x016d      61    PUSH2           0x01a0
0x0170      60    PUSH1           0x04
0x0172      80    DUP1            
0x0173      36    CALLDATASIZE    
0x0174      03    SUB             
0x0175      81    DUP2            
0x0176      01    ADD             
0x0177      90    SWAP1           
0x0178      80    DUP1            
0x0179      80    DUP1            
0x017a      35    CALLDATALOAD    
0x017b      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0190      16    AND             
0x0191      90    SWAP1           
0x0192      60    PUSH1           0x20
0x0194      01    ADD             
0x0195      90    SWAP1           
0x0196      92    SWAP3           
0x0197      91    SWAP2           
0x0198      90    SWAP1           
0x0199      50    POP             
0x019a      50    POP             
0x019b      50    POP             
0x019c      61    PUSH2           0x0360
0x019f      56    JUMP            
0x01a0      5b    JUMPDEST        
0x01a1      00    STOP            
0x01a2      5b    JUMPDEST        
0x01a3      34    CALLVALUE       
0x01a4      80    DUP1            
0x01a5      15    ISZERO          
0x01a6      61    PUSH2           0x01ae
0x01a9      57    JUMPI           
0x01aa      60    PUSH1           0x00
0x01ac      80    DUP1            
0x01ad      fd    REVERT          
0x01ae      5b    JUMPDEST        
0x01af      50    POP             
0x01b0      61    PUSH2           0x01b7
0x01b3      61    PUSH2           0x051e
0x01b6      56    JUMP            
0x01b7      5b    JUMPDEST        
0x01b8      60    PUSH1           0x40
0x01ba      51    MLOAD           
0x01bb      80    DUP1            
0x01bc      82    DUP3            
0x01bd      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x01d2      16    AND             
0x01d3      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x01e8      16    AND             
0x01e9      81    DUP2            
0x01ea      52    MSTORE          
0x01eb      60    PUSH1           0x20
0x01ed      01    ADD             
0x01ee      91    SWAP2           
0x01ef      50    POP             
0x01f0      50    POP             
0x01f1      60    PUSH1           0x40
0x01f3      51    MLOAD           
0x01f4      80    DUP1            
0x01f5      91    SWAP2           
0x01f6      03    SUB             
0x01f7      90    SWAP1           
0x01f8      f3    RETURN          
0x01f9      5b    JUMPDEST        
0x01fa      61    PUSH2           0x0201
0x01fd      61    PUSH2           0x0576
0x0200      56    JUMP            
0x0201      5b    JUMPDEST        
0x0202      61    PUSH2           0x0211
0x0205      61    PUSH2           0x020c
0x0208      61    PUSH2           0x0651
0x020b      56    JUMP            
0x020c      5b    JUMPDEST        
0x020d      61    PUSH2           0x0682
0x0210      56    JUMP            
0x0211      5b    JUMPDEST        
0x0212      56    JUMP            
0x0213      5b    JUMPDEST        
0x0214      61    PUSH2           0x021b
0x0217      61    PUSH2           0x06a8
0x021a      56    JUMP            
0x021b      5b    JUMPDEST        
0x021c      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0231      16    AND             
0x0232      33    CALLER          
0x0233      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0248      16    AND             
0x0249      14    EQ              
0x024a      15    ISZERO          
0x024b      61    PUSH2           0x025c
0x024e      57    JUMPI           
0x024f      61    PUSH2           0x0257
0x0252      81    DUP2            
0x0253      61    PUSH2           0x06d9
0x0256      56    JUMP            
0x0257      5b    JUMPDEST        
0x0258      61    PUSH2           0x0265
0x025b      56    JUMP            
0x025c      5b    JUMPDEST        
0x025d      61    PUSH2           0x0264
0x0260      61    PUSH2           0x01f9
0x0263      56    JUMP            
0x0264      5b    JUMPDEST        
0x0265      5b    JUMPDEST        
0x0266      50    POP             
0x0267      56    JUMP            
0x0268      5b    JUMPDEST        
0x0269      61    PUSH2           0x0270
0x026c      61    PUSH2           0x06a8
0x026f      56    JUMP            
0x0270      5b    JUMPDEST        
0x0271      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0286      16    AND             
0x0287      33    CALLER          
0x0288      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x029d      16    AND             
0x029e      14    EQ              
0x029f      15    ISZERO          
0x02a0      61    PUSH2           0x02fa
0x02a3      57    JUMPI           
0x02a4      61    PUSH2           0x02ac
0x02a7      83    DUP4            
0x02a8      61    PUSH2           0x06d9
0x02ab      56    JUMP            
0x02ac      5b    JUMPDEST        
0x02ad      30    ADDRESS         
0x02ae      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x02c3      16    AND             
0x02c4      34    CALLVALUE       
0x02c5      83    DUP4            
0x02c6      83    DUP4            
0x02c7      60    PUSH1           0x40
0x02c9      51    MLOAD           
0x02ca      80    DUP1            
0x02cb      83    DUP4            
0x02cc      83    DUP4            
0x02cd      80    DUP1            
0x02ce      82    DUP3            
0x02cf      84    DUP5            
0x02d0      37    CALLDATACOPY    
0x02d1      82    DUP3            
0x02d2      01    ADD             
0x02d3      91    SWAP2           
0x02d4      50    POP             
0x02d5      50    POP             
0x02d6      92    SWAP3           
0x02d7      50    POP             
0x02d8      50    POP             
0x02d9      50    POP             
0x02da      60    PUSH1           0x00
0x02dc      60    PUSH1           0x40
0x02de      51    MLOAD           
0x02df      80    DUP1            
0x02e0      83    DUP4            
0x02e1      03    SUB             
0x02e2      81    DUP2            
0x02e3      85    DUP6            
0x02e4      87    DUP8            
0x02e5      5a    GAS             
0x02e6      f1    CALL            
0x02e7      92    SWAP3           
0x02e8      50    POP             
0x02e9      50    POP             
0x02ea      50    POP             
0x02eb      15    ISZERO          
0x02ec      15    ISZERO          
0x02ed      61    PUSH2           0x02f5
0x02f0      57    JUMPI           
0x02f1      60    PUSH1           0x00
0x02f3      80    DUP1            
0x02f4      fd    REVERT          
0x02f5      5b    JUMPDEST        
0x02f6      61    PUSH2           0x0303
0x02f9      56    JUMP            
0x02fa      5b    JUMPDEST        
0x02fb      61    PUSH2           0x0302
0x02fe      61    PUSH2           0x01f9
0x0301      56    JUMP            
0x0302      5b    JUMPDEST        
0x0303      5b    JUMPDEST        
0x0304      50    POP             
0x0305      50    POP             
0x0306      50    POP             
0x0307      56    JUMP            
0x0308      5b    JUMPDEST        
0x0309      60    PUSH1           0x00
0x030b      61    PUSH2           0x0312
0x030e      61    PUSH2           0x06a8
0x0311      56    JUMP            
0x0312      5b    JUMPDEST        
0x0313      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0328      16    AND             
0x0329      33    CALLER          
0x032a      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x033f      16    AND             
0x0340      14    EQ              
0x0341      15    ISZERO          
0x0342      61    PUSH2           0x0354
0x0345      57    JUMPI           
0x0346      61    PUSH2           0x034d
0x0349      61    PUSH2           0x0651
0x034c      56    JUMP            
0x034d      5b    JUMPDEST        
0x034e      90    SWAP1           
0x034f      50    POP             
0x0350      61    PUSH2           0x035d
0x0353      56    JUMP            
0x0354      5b    JUMPDEST        
0x0355      61    PUSH2           0x035c
0x0358      61    PUSH2           0x01f9
0x035b      56    JUMP            
0x035c      5b    JUMPDEST        
0x035d      5b    JUMPDEST        
0x035e      90    SWAP1           
0x035f      56    JUMP            
0x0360      5b    JUMPDEST        
0x0361      61    PUSH2           0x0368
0x0364      61    PUSH2           0x06a8
0x0367      56    JUMP            
0x0368      5b    JUMPDEST        
0x0369      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x037e      16    AND             
0x037f      33    CALLER          
0x0380      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0395      16    AND             
0x0396      14    EQ              
0x0397      15    ISZERO          
0x0398      61    PUSH2           0x0512
0x039b      57    JUMPI           
0x039c      60    PUSH1           0x00
0x039e      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x03b3      16    AND             
0x03b4      81    DUP2            
0x03b5      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x03ca      16    AND             
0x03cb      14    EQ              
0x03cc      15    ISZERO          
0x03cd      15    ISZERO          
0x03ce      15    ISZERO          
0x03cf      61    PUSH2           0x0466
0x03d2      57    JUMPI           
0x03d3      60    PUSH1           0x40
0x03d5      51    MLOAD           
0x03d6      7f    PUSH32          0x08c379a000000000000000000000000000000000000000000000000000000000
0x03f7      81    DUP2            
0x03f8      52    MSTORE          
0x03f9      60    PUSH1           0x04
0x03fb      01    ADD             
0x03fc      80    DUP1            
0x03fd      80    DUP1            
0x03fe      60    PUSH1           0x20
0x0400      01    ADD             
0x0401      82    DUP3            
0x0402      81    DUP2            
0x0403      03    SUB             
0x0404      82    DUP3            
0x0405      52    MSTORE          
0x0406      60    PUSH1           0x36
0x0408      81    DUP2            
0x0409      52    MSTORE          
0x040a      60    PUSH1           0x20
0x040c      01    ADD             
0x040d      80    DUP1            
0x040e      7f    PUSH32          0x43616e6e6f74206368616e6765207468652061646d696e206f6620612070726f
0x042f      81    DUP2            
0x0430      52    MSTORE          
0x0431      60    PUSH1           0x20
0x0433      01    ADD             
0x0434      7f    PUSH32          0x787920746f20746865207a65726f206164647265737300000000000000000000
0x0455      81    DUP2            
0x0456      52    MSTORE          
0x0457      50    POP             
0x0458      60    PUSH1           0x40
0x045a      01    ADD             
0x045b      91    SWAP2           
0x045c      50    POP             
0x045d      50    POP             
0x045e      60    PUSH1           0x40
0x0460      51    MLOAD           
0x0461      80    DUP1            
0x0462      91    SWAP2           
0x0463      03    SUB             
0x0464      90    SWAP1           
0x0465      fd    REVERT          
0x0466      5b    JUMPDEST        
0x0467      7f    PUSH32          0x7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f
0x0488      61    PUSH2           0x048f
0x048b      61    PUSH2           0x06a8
0x048e      56    JUMP            
0x048f      5b    JUMPDEST        
0x0490      82    DUP3            
0x0491      60    PUSH1           0x40
0x0493      51    MLOAD           
0x0494      80    DUP1            
0x0495      83    DUP4            
0x0496      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x04ab      16    AND             
0x04ac      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x04c1      16    AND             
0x04c2      81    DUP2            
0x04c3      52    MSTORE          
0x04c4      60    PUSH1           0x20
0x04c6      01    ADD             
0x04c7      82    DUP3            
0x04c8      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x04dd      16    AND             
0x04de      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x04f3      16    AND             
0x04f4      81    DUP2            
0x04f5      52    MSTORE          
0x04f6      60    PUSH1           0x20
0x04f8      01    ADD             
0x04f9      92    SWAP3           
0x04fa      50    POP             
0x04fb      50    POP             
0x04fc      50    POP             
0x04fd      60    PUSH1           0x40
0x04ff      51    MLOAD           
0x0500      80    DUP1            
0x0501      91    SWAP2           
0x0502      03    SUB             
0x0503      90    SWAP1           
0x0504      a1    LOG1            
0x0505      61    PUSH2           0x050d
0x0508      81    DUP2            
0x0509      61    PUSH2           0x0748
0x050c      56    JUMP            
0x050d      5b    JUMPDEST        
0x050e      61    PUSH2           0x051b
0x0511      56    JUMP            
0x0512      5b    JUMPDEST        
0x0513      61    PUSH2           0x051a
0x0516      61    PUSH2           0x01f9
0x0519      56    JUMP            
0x051a      5b    JUMPDEST        
0x051b      5b    JUMPDEST        
0x051c      50    POP             
0x051d      56    JUMP            
0x051e      5b    JUMPDEST        
0x051f      60    PUSH1           0x00
0x0521      61    PUSH2           0x0528
0x0524      61    PUSH2           0x06a8
0x0527      56    JUMP            
0x0528      5b    JUMPDEST        
0x0529      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x053e      16    AND             
0x053f      33    CALLER          
0x0540      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0555      16    AND             
0x0556      14    EQ              
0x0557      15    ISZERO          
0x0558      61    PUSH2           0x056a
0x055b      57    JUMPI           
0x055c      61    PUSH2           0x0563
0x055f      61    PUSH2           0x06a8
0x0562      56    JUMP            
0x0563      5b    JUMPDEST        
0x0564      90    SWAP1           
0x0565      50    POP             
0x0566      61    PUSH2           0x0573
0x0569      56    JUMP            
0x056a      5b    JUMPDEST        
0x056b      61    PUSH2           0x0572
0x056e      61    PUSH2           0x01f9
0x0571      56    JUMP            
0x0572      5b    JUMPDEST        
0x0573      5b    JUMPDEST        
0x0574      90    SWAP1           
0x0575      56    JUMP            
0x0576      5b    JUMPDEST        
0x0577      61    PUSH2           0x057e
0x057a      61    PUSH2           0x06a8
0x057d      56    JUMP            
0x057e      5b    JUMPDEST        
0x057f      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0594      16    AND             
0x0595      33    CALLER          
0x0596      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x05ab      16    AND             
0x05ac      14    EQ              
0x05ad      15    ISZERO          
0x05ae      15    ISZERO          
0x05af      15    ISZERO          
0x05b0      61    PUSH2           0x0647
0x05b3      57    JUMPI           
0x05b4      60    PUSH1           0x40
0x05b6      51    MLOAD           
0x05b7      7f    PUSH32          0x08c379a000000000000000000000000000000000000000000000000000000000
0x05d8      81    DUP2            
0x05d9      52    MSTORE          
0x05da      60    PUSH1           0x04
0x05dc      01    ADD             
0x05dd      80    DUP1            
0x05de      80    DUP1            
0x05df      60    PUSH1           0x20
0x05e1      01    ADD             
0x05e2      82    DUP3            
0x05e3      81    DUP2            
0x05e4      03    SUB             
0x05e5      82    DUP3            
0x05e6      52    MSTORE          
0x05e7      60    PUSH1           0x32
0x05e9      81    DUP2            
0x05ea      52    MSTORE          
0x05eb      60    PUSH1           0x20
0x05ed      01    ADD             
0x05ee      80    DUP1            
0x05ef      7f    PUSH32          0x43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e206672
0x0610      81    DUP2            
0x0611      52    MSTORE          
0x0612      60    PUSH1           0x20
0x0614      01    ADD             
0x0615      7f    PUSH32          0x6f6d207468652070726f78792061646d696e0000000000000000000000000000
0x0636      81    DUP2            
0x0637      52    MSTORE          
0x0638      50    POP             
0x0639      60    PUSH1           0x40
0x063b      01    ADD             
0x063c      91    SWAP2           
0x063d      50    POP             
0x063e      50    POP             
0x063f      60    PUSH1           0x40
0x0641      51    MLOAD           
0x0642      80    DUP1            
0x0643      91    SWAP2           
0x0644      03    SUB             
0x0645      90    SWAP1           
0x0646      fd    REVERT          
0x0647      5b    JUMPDEST        
0x0648      61    PUSH2           0x064f
0x064b      61    PUSH2           0x0777
0x064e      56    JUMP            
0x064f      5b    JUMPDEST        
0x0650      56    JUMP            
0x0651      5b    JUMPDEST        
0x0652      60    PUSH1           0x00
0x0654      80    DUP1            
0x0655      7f    PUSH32          0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3
0x0676      60    PUSH1           0x01
0x0678      02    MUL             
0x0679      90    SWAP1           
0x067a      50    POP             
0x067b      80    DUP1            
0x067c      54    SLOAD           
0x067d      91    SWAP2           
0x067e      50    POP             
0x067f      50    POP             
0x0680      90    SWAP1           
0x0681      56    JUMP            
0x0682      5b    JUMPDEST        
0x0683      36    CALLDATASIZE    
0x0684      60    PUSH1           0x00
0x0686      80    DUP1            
0x0687      37    CALLDATACOPY    
0x0688      60    PUSH1           0x00
0x068a      80    DUP1            
0x068b      36    CALLDATASIZE    
0x068c      60    PUSH1           0x00
0x068e      84    DUP5            
0x068f      5a    GAS             
0x0690      f4    DELEGATECALL    
0x0691      3d    RETURNDATASIZE  
0x0692      60    PUSH1           0x00
0x0694      80    DUP1            
0x0695      3e    RETURNDATACOPY  
0x0696      80    DUP1            
0x0697      60    PUSH1           0x00
0x0699      81    DUP2            
0x069a      14    EQ              
0x069b      61    PUSH2           0x06a3
0x069e      57    JUMPI           
0x069f      3d    RETURNDATASIZE  
0x06a0      60    PUSH1           0x00
0x06a2      f3    RETURN          
0x06a3      5b    JUMPDEST        
0x06a4      3d    RETURNDATASIZE  
0x06a5      60    PUSH1           0x00
0x06a7      fd    REVERT          
0x06a8      5b    JUMPDEST        
0x06a9      60    PUSH1           0x00
0x06ab      80    DUP1            
0x06ac      7f    PUSH32          0x10d6a54a4754c8869d6886b5f5d7fbfa5b4522237ea5c60d11bc4e7a1ff9390b
0x06cd      60    PUSH1           0x01
0x06cf      02    MUL             
0x06d0      90    SWAP1           
0x06d1      50    POP             
0x06d2      80    DUP1            
0x06d3      54    SLOAD           
0x06d4      91    SWAP2           
0x06d5      50    POP             
0x06d6      50    POP             
0x06d7      90    SWAP1           
0x06d8      56    JUMP            
0x06d9      5b    JUMPDEST        
0x06da      61    PUSH2           0x06e2
0x06dd      81    DUP2            
0x06de      61    PUSH2           0x0779
0x06e1      56    JUMP            
0x06e2      5b    JUMPDEST        
0x06e3      7f    PUSH32          0xbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b
0x0704      81    DUP2            
0x0705      60    PUSH1           0x40
0x0707      51    MLOAD           
0x0708      80    DUP1            
0x0709      82    DUP3            
0x070a      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x071f      16    AND             
0x0720      73    PUSH20          0xffffffffffffffffffffffffffffffffffffffff
0x0735      16    AND             
0x0736      81    DUP2            
0x0737      52    MSTORE          
0x0738      60    PUSH1           0x20
0x073a      01    ADD             
0x073b      91    SWAP2           
0x073c      50    POP             
0x073d      50    POP             
0x073e      60    PUSH1           0x40
0x0740      51    MLOAD           
0x0741      80    DUP1            
0x0742      91    SWAP2           
0x0743      03    SUB             
0x0744      90    SWAP1           
0x0745      a1    LOG1            
0x0746      50    POP             
0x0747      56    JUMP            
0x0748      5b    JUMPDEST        
0x0749      60    PUSH1           0x00
0x074b      7f    PUSH32          0x10d6a54a4754c8869d6886b5f5d7fbfa5b4522237ea5c60d11bc4e7a1ff9390b
0x076c      60    PUSH1           0x01
0x076e      02    MUL             
0x076f      90    SWAP1           
0x0770      50    POP             
0x0771      81    DUP2            
0x0772      81    DUP2            
0x0773      55    SSTORE          
0x0774      50    POP             
0x0775      50    POP             
0x0776      56    JUMP            
0x0777      5b    JUMPDEST        
0x0778      56    JUMP            
0x0779      5b    JUMPDEST        
0x077a      60    PUSH1           0x00
0x077c      61    PUSH2           0x0784
0x077f      82    DUP3            
0x0780      61    PUSH2           0x084b
0x0783      56    JUMP            
0x0784      5b    JUMPDEST        
0x0785      15    ISZERO          
0x0786      15    ISZERO          
0x0787      61    PUSH2           0x081e
0x078a      57    JUMPI           
0x078b      60    PUSH1           0x40
0x078d      51    MLOAD           
0x078e      7f    PUSH32          0x08c379a000000000000000000000000000000000000000000000000000000000
0x07af      81    DUP2            
0x07b0      52    MSTORE          
0x07b1      60    PUSH1           0x04
0x07b3      01    ADD             
0x07b4      80    DUP1            
0x07b5      80    DUP1            
0x07b6      60    PUSH1           0x20
0x07b8      01    ADD             
0x07b9      82    DUP3            
0x07ba      81    DUP2            
0x07bb      03    SUB             
0x07bc      82    DUP3            
0x07bd      52    MSTORE          
0x07be      60    PUSH1           0x3b
0x07c0      81    DUP2            
0x07c1      52    MSTORE          
0x07c2      60    PUSH1           0x20
0x07c4      01    ADD             
0x07c5      80    DUP1            
0x07c6      7f    PUSH32          0x43616e6e6f742073657420612070726f787920696d706c656d656e746174696f
0x07e7      81    DUP2            
0x07e8      52    MSTORE          
0x07e9      60    PUSH1           0x20
0x07eb      01    ADD             
0x07ec      7f    PUSH32          0x6e20746f2061206e6f6e2d636f6e747261637420616464726573730000000000
0x080d      81    DUP2            
0x080e      52    MSTORE          
0x080f      50    POP             
0x0810      60    PUSH1           0x40
0x0812      01    ADD             
0x0813      91    SWAP2           
0x0814      50    POP             
0x0815      50    POP             
0x0816      60    PUSH1           0x40
0x0818      51    MLOAD           
0x0819      80    DUP1            
0x081a      91    SWAP2           
0x081b      03    SUB             
0x081c      90    SWAP1           
0x081d      fd    REVERT          
0x081e      5b    JUMPDEST        
0x081f      7f    PUSH32          0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3
0x0840      60    PUSH1           0x01
0x0842      02    MUL             
0x0843      90    SWAP1           
0x0844      50    POP             
0x0845      81    DUP2            
0x0846      81    DUP2            
0x0847      55    SSTORE          
0x0848      50    POP             
0x0849      50    POP             
0x084a      56    JUMP            
0x084b      5b    JUMPDEST        
0x084c      60    PUSH1           0x00
0x084e      80    DUP1            
0x084f      82    DUP3            
0x0850      3b    EXTCODESIZE     
0x0851      90    SWAP1           
0x0852      50    POP             
0x0853      60    PUSH1           0x00
0x0855      81    DUP2            
0x0856      11    GT              
0x0857      91    SWAP2           
0x0858      50    POP             
0x0859      50    POP             
0x085a      91    SWAP2           
0x085b      90    SWAP1           
0x085c      50    POP             
0x085d      56    JUMP            
0x085e      00    STOP            
0x085f      a1    LOG1            
0x0860      65    PUSH6           0x627a7a723058
0x0867      20    KECCAK256       
0x0868      a4    LOG4            
0x0869      a5    UNKNOWN         
0x086a      47    SELFBALANCE     
0x086b      cf    UNKNOWN         
0x086c      c7    UNKNOWN         
0x086d      20    KECCAK256       
0x086e      2c    UNKNOWN         
0x086f      5a    GAS             
0x0870      ca    UNKNOWN         
0x0871      aa    UNKNOWN         
0x0872      e7    UNKNOWN         
0x0873      4d    UNKNOWN         
0x0874      42    TIMESTAMP       
0x0875      8e    DUP15           
0x0876      98    SWAP9           
0x0877      8b    DUP12           
0x0878      c6    UNKNOWN         
0x0879      2a    UNKNOWN         
0x087a      d5    UNKNOWN         
0x087b      02    MUL             
0x087c      4e    UNKNOWN         
0x087d      b0    UNKNOWN         
0x087e      16    AND             
0x087f      55    SSTORE          
0x0880      32    ORIGIN          
0x0881      d3    UNKNOWN         
0x0882      a8    UNKNOWN         
0x0883      f9    UNKNOWN         
0x0884      1d    SAR             
0x0885      b4    UNKNOWN         
0x0886      ed    UNKNOWN         
0x0887      24    UNKNOWN         
0x0888      00    STOP            
0x0889      29    UNKNOWN          
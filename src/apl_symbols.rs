// TODO: Whenever possible in stable const rust: convert this to a map
pub const APL_SYMBOLS: &[(char, char, char)] = &[
	('l', 'l', '⌊'),
	('a', 'a', '⍺'),
	('7', '7', '⌈'),
	('F', 'F', '⌈'),
	('o', 'n', '⍝'),
	('~', '\"', '⍨'),
	('~',':', '⍨'),
	('|','_', '⊥'),
	('=','=', '≡'),
	('=','_', '≡'),
	(':','-', '÷'),
	(':',':', '¨'),
	('\"','\"', '¨'),
	('0','~', '⍬'),
	('~','0', '⍬'),
	('(','(', '⊂'),
	('c','c', '⊂'),
	('T','T', '⊤'),
	('t','t', '⊤'),
	('e','e', '∊'),
	('o','_', '⍎'),
	('_','o', '⍎'),
	('\\','-', '⍀'),
	('e','_', '⍷'),
	(')',')', '⊃'),
	('l','l', '⌊'),
	('L','L', '⌊'),
	('o','T', '⍕'),
	('o','-', '⍕'),
	('v','v', '∨'),
	('V','|', '⍒'),
	('|','V', '⍒'),
	('A','|', '⍋'),
	('|','A', '⍋'),
	('>','=', '≥'),
	('>','_', '≥'),
	('T','_', '⌶'),
	('|','|', '⌶'),
	('[','|', '⌷'),
	('|',']', '⌷'),
	('[','=', '⌸'),
	('=',']', '⌸'),
	('i','i', '⍳'),
	('n','n', '∩'),
	('^','^', '∧'),
	('<','-', '←'),
	('<','=', '≤'),
	('<','_', '≤'),
	('*','O', '⍟'),
	('[','-', '⌹'),
	('-',']', '⌹'),
	('^','|', '↑'),
	('^','~', '⍲'),
	('-','-', '¯'),
	('c','_', '⊆'),
	('(','_', '⊆'),
	('v','~', '⍱'),
	('!','=', '≠'),
	('=','/', '≠'),
	('L','-', '≠'),
	('w','w', '⍵'),
	('o','o', '∘'),
	('O',':', '⍥'),
	('O','\"', '⍥'),
	('o','o', '○'),
	('o',':', '⍤'),
	('o','\"', '⍤'),
	('V','V', '∇'),
	('v','-', '∇'),
	('*',':', '⍣'),
	('*','\"', '⍣'),
	('/','-', '⌿'),
	('O','|', '⌽'),
	('|','O', '⌽'),
	('O','-', '⊖'),
	('-','O', '⊖'),
	('-','>', '→'),
	('-','|', '⊣'),
	('|','-', '⊢'),
	('r','r', '⍴'),
	('p','p', '⍴'),
	('v','|', '↓'),
	('<','>', '⋄'),
	('^','v', '⋄'),
	('[','<', '⌺'),
	('>',']', '⌺'),
	('[',']', '⎕'),
	(',','-', '⍪'),
	('7','=', '≢'),
	('Z','-', '≢'),
	('x','x', '×'),
	('O','\\', '⍉'),
	('\\','O', '⍉'),
	('u','u', '∪'),
	('U','U', '∪'),
	('[',':', '⍠'),
	(':',']', '⍠'),
	('i','_', '⍸'),
];

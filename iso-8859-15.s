;
;      ISO-8859-15        X16 Additions
;                           (inverted)
;
; 00|                | |@ABCDEFGHIJKLMNO|
; 10|                | |PQRSTUVWXYZ[\]^_|
; 20| !"#$%&'()*+,-./|
; 30|0123456789:;<=>?|
; 40|@ABCDEFGHIJKLMNO|
; 50|PQRSTUVWXYZ[\]^_|
; 60|`abcdefghijklmno|
; 70|pqrstuvwxyz{|}~ |
; 80|                | |`abcdefghijklmno|
; 90|                | |pqrstuvwxyz{|}~ |
; A0| ¡¢£€¥Š§š©ª«¬ ®¯|
; B0|°±²³Žµ¶·ž¹º»ŒœŸ¿|
; C0|ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏ|
; D0|ÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞß|
; E0|àáâãäåæçèéêëìíîï|
; F0|ðñòóôõö÷øùúûüýþÿ|

.segment "CHARISO"

.incbin "Japanese.chr"
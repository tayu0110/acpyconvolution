from enum import Enum
from typing import Iterable

class KnownModulo(Enum): ...

M167772161: KnownModulo
M377487361: KnownModulo
M469762049: KnownModulo
M595591169: KnownModulo
M645922817: KnownModulo
M754974721: KnownModulo
M880803841: KnownModulo
M897581057: KnownModulo
M998244353: KnownModulo
M1107296257: KnownModulo
M1224736769: KnownModulo
M1300234241: KnownModulo
M1484783617: KnownModulo
M1711276033: KnownModulo
M1811939329: KnownModulo
M2013265921: KnownModulo
M2088763393: KnownModulo
M2113929217: KnownModulo
M2130706433: KnownModulo
M2281701377: KnownModulo
M2483027969: KnownModulo
M2533359617: KnownModulo
M2634022913: KnownModulo
M2717908993: KnownModulo
M2868903937: KnownModulo
M2885681153: KnownModulo
M3221225473: KnownModulo
M3238002689: KnownModulo
M3489660929: KnownModulo
M3892314113: KnownModulo
M3942645761: KnownModulo
M4076863489: KnownModulo
M4194304001: KnownModulo

def convolution(
    a: Iterable[int], b: Iterable[int], modulo: KnownModulo
) -> list[int]: ...

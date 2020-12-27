
```plantuml { format="txt" }

class Ipam
class CidrEntry
class Label
Ipam "1" *-- "many" CidrEntry
CidrEntry "1" *-- "many" Label

```

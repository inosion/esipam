#
#
#
PADOUT        = awk '{printf "%$(1)s%s\n", " ", $$0}'

dirguard      = @mkdir -p $(@D)

# 
# This is the Help Support
#
# Original is from: https://codereview.stackexchange.com/questions/94307/built-in-help-in-a-makefile
#
# Howto:
# Label the `target: ` with a double hashed (##) comment. @category prefix is optional.
#

.PHONY: help
help: ## @other Show this help.
	@perl -e '$(HELP_FUNC)' $(MAKEFILE_LIST)

# Colours for the Help
#
GREEN   := $(shell tput -Txterm setaf 2)
WHITE   := $(shell tput -Txterm setaf 7)
YELLOW  := $(shell tput -Txterm setaf 3)
RESET   := $(shell tput -Txterm sgr0)

#
# main function in Perl, which reads the Makefile, extracting the Help text
#
HELP_FUNC = \
	while(<>) {                                                                        \
	    push @{$$hyy{$$2 // 'options'}}, [$$1, $$3] if                                \
              /^([\$$\(\)a-zA-Z0-9\-\.%\/_]+)\s*:.*\#\#(?:\s?@([a-zA-Z0-9_\-]+))?\s(.*)$$/ \
	};                                                                                 \
	print "usage make [target]\n\n";                                                   \
	for (sort keys %hyy) {                                                            \
	  print "${WHITE}$$_:${RESET}\n";                                                  \
	  for (@{$$hyy{$$_}}) {                                                           \
	    $$sep = " " x (32 - length $$_->[0]);                                          \
	    print " ${YELLOW}$$_->[0]${RESET}$$sep${GREEN}$$_->[1]${RESET}\n";             \
	  };                                                                               \
	  print "\n";                                                                      \
	}


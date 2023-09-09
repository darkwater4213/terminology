#!/bin/sh

echo "\\e[XXm\n"

echo "\e[1m"
echo "\e[30m30\t\t\e[90m90\n"
echo "\e[31m31\t\t\e[91m91\n"
echo "\e[32m32\t\t\e[92m92\n"
echo "\e[33m33\t\t\e[93m93\n"
echo "\e[34m34\t\t\e[94m94\n"
echo "\e[35m35\t\t\e[95m95\n"
echo "\e[36m36\t\t\e[96m96\n"
echo "\e[37m37\t\t\e[97m97\n"

echo "\e[0m"
echo "\\e[49m - Reset colour\n"
echo "\\e[2K - Clear Line\n"
echo "\\e[<L>;<C>H OR \\e[<L>;<C>f puts the cursor at line L and column C.\n"
echo "\\e[<N>A Move the cursor up N lines\n"
echo "\\e[<N>B Move the cursor down N lines\n"
echo "\\e[<N>C Move the cursor forward N columns\n"
echo "\\e[<N>D Move the cursor backward N columns\n"
echo "\\e[2J Clear the screen, move to (0,0)\n"
echo "\\e[K Erase to end of line\n"
echo "\\e[s Save cursor position\n"
echo "\\e[u Restore cursor position\n"
echo "\n"
echo "\\e[4m  Underline on\n"
echo "\\e[24m Underline off\n"
echo "\\e[1m  Bold on\n"
echo "\\e[21m Bold off\n"

# ANSI colors
ANSI_RESET="\e[0m";	ANSI_COLOR="\e[49m";
ANSI_BLACK="\e[30m";	ANSI_BLACKB="\e[90m";
ANSI_RED="\e[31m";	ANSI_REDB="\e[91m";
ANSI_GREEN="\e[32m";	ANSI_GREENB="\e[92m";
ANSI_YELLOW="\e[33m";	ANSI_YELLOWB="\e[93m";
ANSI_BLUE="\e[34m";	ANSI_BLUEB="\e[94m";
ANSI_PURPLE="\e[35m";	ANSI_PURPLEB="\e[95m";
ANSI_CYAN="\e[36m";	ANSI_CYANB="\e[96m";
ANSI_WHITE="\e[37m";	ANSI_WHITEB="\e[97m";

# ANSI text
ANSI_BOLD="\e[1m";	ANSI_BOLD_="\e[21m";
ANSI_DIM="\e[2m";	 	ANSI_DIM_="\e[22m";
ANSI_ITALIC="\e[3m";	ANSI_ITALIC_="\e[23m";
ANSI_UL="\e[4m";		ANSI_UL_="\e[24m";

# echo "# ANSI colors
# ANSI_RESET=\"\\e[0m\";	ANSI_COLOR=\"\\e[49m\";
# ANSI_BLACK=\"\\e[30m\";	ANSI_BLACKB=\"\\e[90m\";
# ANSI_RED=\"\\e[31m\";	ANSI_REDB=\"\\e[91m\";
# ANSI_GREEN=\"\\e[32m\";	ANSI_GREENB=\"\\e[92m\";
# ANSI_YELLOW=\"\\e[33m\";	ANSI_YELLOWB=\"\\e[93m\";
# ANSI_BLUE=\"\\e[34m\";	ANSI_BLUEB=\"\\e[94m\";
# ANSI_PURPLE=\"\\e[35m\";	ANSI_PURPLEB=\"\\e[95m\";
# ANSI_CYAN=\"\\e[36m\";	ANSI_CYANB=\"\\e[96m\";
# ANSI_WHITE=\"\\e[37m\";	ANSI_WHITEB=\"\\e[97m\";
# 
# # ANSI text
# ANSI_BOLD=\"\\e[1m\";	ANSI_BOLD_=\"\\e[21m\";
# ANSI_DIM=\"\\e[2m\";	 	ANSI_DIM_=\"\\e[22m\";
# ANSI_ITALIC=\"\\e[3m\";	ANSI_ITALIC_=\"\\e[23m\";
# ANSI_UL=\"\\e[4m\";		ANSI_UL_=\"\\e[24m\"";
# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get mecha-adc could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/mecha-adc/0.1.0"
SRC_URI += "git://github.com/Dhruvesh08/mecha-adc.git;protocol=https;nobranch=1;branch=mecha-adc-v1"
SRCREV = "d0b95505554067e7c22b22e383cc59f13017de36"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+d0b9550555"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
"



# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "mecha-adc"
HOMEPAGE = "https://github.com/Dhruvesh08/mecha-adc"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include mecha-adc-${PV}.inc
include mecha-adc.inc

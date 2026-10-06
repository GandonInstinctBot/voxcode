# harness

Local-first voice agent harness for code projects. Early scaffold: self-update works, everything else is coming.

    harness                 # prints version
    harness update --check  # look for a newer GitHub release
    harness update          # download and replace the running binary

Every push to main builds Linux, macOS and Windows binaries and publishes a release.

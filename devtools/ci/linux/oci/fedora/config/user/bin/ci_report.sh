#!/bin/sh -
# This script is called by Task Spooler once a CI job has finished.

export job_id="$1"
export error_code="$2"
export out_file="$3"
export command="$4"

export subject_subtitle

email_body()
{
    echo "Task Spooler Job ID: $job_id"
    echo "Task Spooler Command: $command"
    echo
    cat "$out_file"
}

if [ "$error_code" == "0" ]; then
    subject_subtitle="Ok."
else
    subject_subtitle="ERROR!"
fi

email_body | mailx -s "PhasicJ Fedora CI Results: $subject_subtitle" phasicj-ci

"""Use picocli's built-in doc gen tool & Asciidoctor to make `phasicj` CLI docs.
"""

MAN_PAGE_GENERATOR = "picocli.codegen.docgen.manpage.ManPageGenerator"
COMMAND_TO_BE_DOCUMENTED = "phasicj.cli.cmd.Pj"
ASCIIDOC_FILES = [
    "phasicj.adoc",
    "phasicj-agent.adoc",
    "phasicj-agent-instrument-class.adoc",
]
ASCIIDOCTOR_EXEC = "asciidoctor"
HTML_FILES = [
    "phasicj.html",
    "phasicj-agent.html",
    "phasicj-agent-instrument-class.html",
]

# TODO(dwtj): Don't use a local `java`.
ASCIIDOC_BASH_CMD = """\
    java \
        --class-path "$(execpath //third_party/picocli:picocli_jar):$(execpath //third_party/picocli:picocli_codegen_jar):$(execpath //phasicj/cli/cmd)" \
        {0} \
        "--outdir=`dirname $(execpath :{1})`" \
        {2} \
    """.format(
        MAN_PAGE_GENERATOR,
        ASCIIDOC_FILES[0],
        COMMAND_TO_BE_DOCUMENTED,
    )

genrule(
    name = "asciidoc",
    srcs = [
        "//third_party/picocli:picocli_jar",
        "//third_party/picocli:picocli_codegen_jar",
        "//phasicj/cli/cmd",
    ],
    cmd_bash = ASCIIDOC_BASH_CMD,
    outs = ASCIIDOC_FILES,
)

HTML_BASH_CMD = """\
    {0} \
        --backend html5 \
        --doctype manpage \
        --destination-dir `dirname $(execpath :{1})` \
        $(SRCS) \
""".format(
    ASCIIDOCTOR_EXEC,
    HTML_FILES[0],
)

genrule(
    name = "html",
    srcs = ASCIIDOC_FILES,
    cmd_bash = HTML_BASH_CMD,
    outs = HTML_FILES,
)

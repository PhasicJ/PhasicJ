JAVA_EXEC_LABEL="//third_party/openjdk:java"
PHASICJ_AGENT_LABEL="//phasicj/agent:libpjagent"
RENAISSANCE_JAR_LABEL="@com_github_renaissance_benchmarks//jar"
RENAISSANCE_MAIN_CLASS="org.renaissance.core.Launcher"

sh_test(
    name = "scrabble",
    srcs = ["test.sh"],
    data = [
        JAVA_EXEC_LABEL,
        PHASICJ_AGENT_LABEL,
        RENAISSANCE_JAR_LABEL,
    ],
    args = [
        "$(rootpath {})".format(JAVA_EXEC_LABEL),
        "$(rootpath {})".format(PHASICJ_AGENT_LABEL),
        "$(rootpath {})".format(RENAISSANCE_JAR_LABEL),
        RENAISSANCE_MAIN_CLASS,
        "--repetitions",
        "1",
        "scrabble"
    ],
)

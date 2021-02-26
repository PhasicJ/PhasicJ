JAVA_EXEC_LABEL="//third_party/openjdk:java"
PHASICJ_AGENT_LABEL="//phasicj/agent:libpjagent"
RENAISSANCE_JAR_LABEL="@com_github_renaissance_benchmarks//jar"
RENAISSANCE_MAIN_CLASS="org.renaissance.core.Launcher"
PHASICJ_EXEC="//phasicj/cli"
EXTRA_PHASICJ_AGENT_OPTIONS="verbose"

def smoke_test_benchmark(name):
    native.sh_test(
        name = name,
        srcs = ["test.sh"],
        data = [
            JAVA_EXEC_LABEL,
            PHASICJ_AGENT_LABEL,
            RENAISSANCE_JAR_LABEL,
            PHASICJ_EXEC,
        ],
        args = [
            "$(rootpath {})".format(JAVA_EXEC_LABEL),
            "$(rootpath {})".format(PHASICJ_AGENT_LABEL),
            "$(rootpath {})".format(RENAISSANCE_JAR_LABEL),
            RENAISSANCE_MAIN_CLASS,
            "$(rootpath {})".format(PHASICJ_EXEC),
            EXTRA_PHASICJ_AGENT_OPTIONS,
            "--repetitions",
            "1",
            name
        ],
    )

KNOWN_BENCHMARKS = [
    "akka-uct",
    "als",
    "chi-square",
    # TODO(dwtj): Figure out why this crashes.
    # "db-shootout",
    "dec-tree",
    "dotty",
    "finagle-chirper",
    "finagle-http",
    "fj-kmeans",
    "future-genetic",
    "gauss-mix",
    "log-regression",
    "mnemonics",
    "movie-lens",
    "naive-bayes",
    "neo4j-analytics",
    "page-rank",
    "par-mnemonics",
    "philosophers",
    "reactors",
    "rx-scrabble",
    "scala-doku",
    "scala-kmeans",
    "scala-stm-bench7",
    "scrabble",
]

def test_each_known_renaissance_benchmark():
    for benchmark in KNOWN_BENCHMARKS:
        smoke_test_benchmark(benchmark)

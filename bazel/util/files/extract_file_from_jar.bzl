"""Defines the `extract_file_from_jar()` macro.
"""

def extract_file_from_jar(
        name,
        in_jar,
        file_to_extract,
        out_file,
        visibility = [":__pkg__"]):
    """Declares a `genrule` which uses JAR to extract a file from a JAR file.

    Uses the JAR command from the resolved `@dwtj_rules_java` Java toolchain.

    Args:
      name: The name of the genrule which actually performs the extraction.
      in_jar: The JAR file from which we are extracting a file.
      file_to_extract: A path within the JAR (relative to the JAR's root) to
        the file to be extracted.
      out_file: A path within the declaring package where the extracted file
        will be placed.
      visibility: The visibility of the genrule.
    """
    native.genrule(
        name = name,
        srcs = [in_jar],
        outs = [out_file],
        cmd_bash = '''"$(rootpath //third_party/openjdk:jar)" -xf "$<" {0} && mv --no-clobber "{0}" "$@"'''.format(file_to_extract),
        tools = ["//third_party/openjdk:jar"],
        visibility = visibility,
    )

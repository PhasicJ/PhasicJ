#ifndef _PT_MONITORENTER_COUNT_AGENT_RT_EXTRACT_EMBEDDED_JAR_H
#define _PT_MONITORENTER_COUNT_AGENT_RT_EXTRACT_EMBEDDED_JAR_H

#include <cstddef>
#include <cstdint>

namespace pt::monitorenter::count::agent::rt {

const std::byte* jar_start_ptr();
size_t jar_size();
const std::byte* jar_end_ptr();
void write_jar_to_path(std::filesystem::path jar_path);
std::filesystem::path write_jar_to_temp();

}  // namespace pt::monitorenter::count::agent::rt

#endif  // _PT_MONITORENTER_COUNT_AGENT_RT_EXTRACT_EMBEDDED_JAR_H

#include <Poco/Net/ICMPClient.h>
#include <chrono>
#include <fmt/core.h>
#include <iostream>
#include <tbb/tbb.h>
#include <thread>
#include <vector>

using namespace fmt;
using namespace std::chrono_literals;
using namespace Poco;

void ping(const std::string_view target) {
  try {
    Net::ICMPClient client(Net::AddressFamily::IPv4);
    const auto times = client.ping(target.data());
    if (times)
      std::cout << target << std::endl;
  } catch (const Poco::Exception &e) {
    std::cerr << e.displayText() << std::endl;
  }
}

int main() {
  std::vector<std::string> targets;
  using IterType = decltype(targets)::const_iterator;

  std::string input;
  while (std::getline(std::cin, input))
    targets.push_back(input);

  auto par_for = [](const tbb::blocked_range<IterType> &range) {
    for (const auto &target : range) {
      ping(target);
    }
  };
  tbb::parallel_for(
      tbb::blocked_range<IterType>(targets.begin(), targets.end()), par_for);
}

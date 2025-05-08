// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "common/mem_pool.h"

#include <gtest/gtest.h>

namespace zen::test {

using namespace zen;
using namespace common;

TEST(Mempool, CodeMemPool) {
  CodeMemPool Pool;
  auto *Start = Pool.getMemStart();
  EXPECT_EQ(Pool.getMemStart(), Start);
  EXPECT_EQ(Pool.getMemEnd(), Start);
  EXPECT_EQ(Pool.getMemPageEnd(), Start);

  void *Ptr = Pool.allocate(10);
  EXPECT_EQ(Pool.getMemStart(), Start);
  EXPECT_EQ(Pool.getMemEnd(), Start + 10);
  EXPECT_EQ(Ptr, Start);
  EXPECT_EQ(Pool.getMemPageEnd(), Start + 4096);

  Ptr = Pool.allocate(10);
  EXPECT_EQ(Pool.getMemStart(), Start);
  EXPECT_EQ(Pool.getMemEnd(), Start + 26);
  EXPECT_EQ(Ptr, Start + 16);
  EXPECT_EQ(Pool.getMemPageEnd(), Start + 4096);

  Ptr = Pool.allocate(4096);
  EXPECT_EQ(Pool.getMemStart(), Start);
  EXPECT_EQ(Pool.getMemEnd(), Start + 32 + 4096);
  EXPECT_EQ(Ptr, Start + 32);
  EXPECT_EQ(Pool.getMemPageEnd(), Start + 4096 * 2);

  EXPECT_DEATH(Pool.allocate(CodeMemPool::MaxCodeSize), "");
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}

} // namespace zen::test

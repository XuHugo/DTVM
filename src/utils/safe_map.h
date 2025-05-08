// Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef ZEN_UTILS_SAFE_MAP_H
#define ZEN_UTILS_SAFE_MAP_H

#include "common/defines.h"
#include <functional>
#include <map>
#include <thread>

namespace zen::utils {

/**
 * get thread-related unique id
 * @return
 */
int64_t getThreadLocalUniqueId();

template <typename Key, typename Value, class Compare = std::less<Key>,
          class Alloc = std::allocator<std::pair<const Key, Value>>>
class ThreadSafeMap {
  typedef typename std::map<Key, Value, Compare, Alloc>::iterator iterator;
  typedef typename std::map<Key, Value, Compare, Alloc>::const_iterator
      const_iterator;
  typedef typename std::map<Key, Value, Compare, Alloc>::allocator_type
      allocator_type;
  typedef
      typename std::map<Key, Value, Compare, Alloc>::mapped_type mapped_type;
  typedef typename std::map<Key, Value, Compare, Alloc>::size_type size_type;
  typedef typename std::map<Key, Value, Compare, Alloc> map;

  typedef common::SharedMutex SharedMutex;
  typedef common::UniqueLock<common::SharedMutex> WriteLock;
  typedef common::SharedLock<common::SharedMutex> ReadLock;

public:
  virtual ~ThreadSafeMap() {}

  bool empty() noexcept {
    ReadLock RLock(Mutex);
    return Data.empty();
  }

  size_type size() noexcept {
    ReadLock RLock(Mutex);
    return Data.size();
  }

  Value &operator[](const Key &K) {
    WriteLock WLock(Mutex);
    return Data.operator[](K);
  }

  Value &operator[](Key &&K) {
    WriteLock WLock(Mutex);
    return Data.operator[](K);
  }

  const Value &at(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.at(K);
  }

  void put(const Key &K, const Value &Val) {
    WriteLock WLock(Mutex);
    Data[K] = Val;
  }

  Value get(const Key &K) {
    ReadLock RLock(Mutex);
    return Data[K];
  }

  std::pair<iterator, bool> insert(const Value &Val) {
    WriteLock WLock(Mutex);
    return Data.insert(Val);
  }

  std::pair<iterator, bool> insert(Value &&Val) {
    WriteLock WLock(Mutex);
    return Data.insert(std::move(Val));
  }

  iterator insert(iterator Position, const Value &Val) {
    WriteLock WLock(Mutex);
    return Data.insert(Position, Val);
  }

  iterator insert(iterator Position, Value &&Val) {
    WriteLock WLock(Mutex);
    return Data.insert(Position, std::move(Val));
  }

  template <class InputIterator>
  void insert(InputIterator First, InputIterator Last) {
    WriteLock WLock(Mutex);
    return Data.insert(First, Last);
  }

  iterator erase(const_iterator Position) {
    WriteLock WLock(Mutex);
    return Data.erase(Position);
  }

  size_type erase(const Key &K) {
    WriteLock WLock(Mutex);
    return Data.erase(K);
  }

  iterator erase(const_iterator First, const_iterator Last) {
    WriteLock WLock(Mutex);
    return Data.erase(First, Last);
  }

  void swap(map &Other) {
    WriteLock WLock(Mutex);
    Data.swap(Other);
  }

  void swap(ThreadSafeMap<Key, Value, Compare, Alloc> &Other) {
    WriteLock WLock(Mutex);
    WriteLock XWriteLock(Other.Mutex);
    Data.swap(Other);
  }

  void clear() noexcept {
    WriteLock WLock(Mutex);
    Data.clear();
  }

  void each(std::function<void(Key, Value)> Handler) {
    ReadLock RLock(Mutex);
    for (const auto &P : Data) {
      Handler(P.first, P.second);
    }
  }

  template <class... Arguments>
  std::pair<iterator, bool> emplace(Arguments &&...Args) {
    WriteLock WLock(Mutex);
    return Data.emplace(Args...);
  }

  template <class... Arguments>
  iterator emplaceHint(const_iterator Position, Arguments &&...Args) {
    WriteLock WLock(Mutex);
    return Data.emplace_hint(Position, Args...);
  }

  const_iterator find(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.find(K);
  }

  const_iterator end() {
    ReadLock RLock(Mutex);
    return Data.end();
  }

  bool containsKey(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.find(K) != Data.end();
  }

  size_type count(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.count(K);
  }

  const_iterator lowerBound(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.lower_bound(K);
  }

  const_iterator upperBound(const Key &K) {
    ReadLock RLock(Mutex);
    return Data.upper_bound(K);
  }

private:
  SharedMutex Mutex;
  std::map<Key, Value, Compare, Alloc> Data;
};

} // namespace zen::utils

#endif // ZEN_UTILS_SAFE_MAP_H

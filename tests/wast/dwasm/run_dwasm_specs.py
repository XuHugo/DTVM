#!/usr/bin/env python3

# Copyright (C) 2021-2023 the DTVM authors. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

import os
import subprocess

def get_spec_tests_args():
    result = os.environ.get('SPEC_TESTS_ARGS')
    assert result is not None, 'env SPEC_TESTS_ARGS is not set'
    return result

def match_interpreter_mode():
    return get_spec_tests_args().find('-m interpreter') >= 0 or get_spec_tests_args().find('-m 0') >= 0

def match_singlepass_mode():
    return get_spec_tests_args().find('-m singlepass') >= 0 or get_spec_tests_args().find('-m 1') >= 0

def is_disabled_interpreter(wast_code):
    return wast_code.find(';; disabled') >= 0 or wast_code.find(';; interpreter-disabled') >= 0

def is_disabled_singlepass(wast_code):
    return wast_code.find(';; disabled') >= 0 or wast_code.find(';; singlepass-disabled') >= 0

def is_case_disabled(wast_code):
    if match_interpreter_mode():
        return is_disabled_interpreter(wast_code)
    if match_singlepass_mode():
        return is_disabled_singlepass(wast_code)
    return False


def run_case(case_name):
    print(f'running case {case_name}')
    case_wast = f'{case_name}.wast'
    with open(case_wast) as f:
        wast_code = f.read()
        if is_case_disabled(wast_code):
            print(f'\033[33mcase {case_name} disabled\033[0m\n')
            return True

    command = 'specUnitTests {} -c dwasm {}'.format(get_spec_tests_args(), case_name)
    p = subprocess.Popen(
      command.split(),
      stdout=subprocess.PIPE,
      stderr=subprocess.PIPE
    )
    stdout, stderr = p.communicate()
    out = stdout.decode('utf-8')
    err = stderr.decode('utf-8')
    sub_exit_code = p.returncode
    outputs = out + err
    print(outputs)
    if sub_exit_code != 0 or outputs.find('Failure') >= 0:
        print(f'\033[31mcase {case_name} failed\033[0m\n')
        return False
    print(f'\033[32mcase {case_name} passed\033[0m\n')
    return True


def main():
    all_cases_count = 0
    passed_cases_count = 0
    for file in os.listdir('.'):
        wat_name = os.path.basename(file)
        if not wat_name.endswith('.wast'):
            continue
        success = run_case(wat_name[0:len(wat_name) - len('.wast')])
        all_cases_count += 1
        if success:
            passed_cases_count += 1
    print(f'\n{passed_cases_count} cases passed, {all_cases_count - passed_cases_count}'
          f' cases failed, total {all_cases_count} cases')
    if all_cases_count > passed_cases_count:
        exit(1)


if __name__ == '__main__':
    main()

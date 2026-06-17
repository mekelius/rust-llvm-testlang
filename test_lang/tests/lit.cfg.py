import lit.formats

config.name = 'testl'
config.test_format = lit.formats.ShTest(True)

config.suffixes = ['.test']

config.test_source_root = os.path.dirname(__file__)
config.test_exec_root = os.path.join(config.test_source_root, 'test_exec')

config.substitutions.append(('%testc', 'cargo -q run --bin testc 2>/dev/null'))

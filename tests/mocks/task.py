import sys

arg = sys.argv[1] if len(sys.argv) == 2 else ""
sys.stdout.write("test_okey {0}".format(arg))
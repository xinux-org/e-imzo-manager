global_conf = configuration_data()
# cargo_options = [ '--manifest-path', meson.project_source_root() / 'e-helper' /  'Cargo.toml' ]
# cargo_options += [ '--target-dir', meson.project_build_root() / 'e-helper' /  'src' ]

# if get_option('profile') == 'default'
#   cargo_options += [ '--release' ]
#   rust_target = 'release'
#   message('Building in release mode')
# else
#   rust_target = 'debug'
#   message('Building in debug mode')
# endif

# cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'e-helper' / 'cargo-home' ]

# cargo_build = custom_target(
#   'cargo-build',
#   build_by_default: true,
#   build_always_stale: true,
#   output: 'e-helper',
#   console: true,
#   install: true,
#   install_dir: get_option('libexecdir'),
#   command: [
#     'env',
#     cargo_env,
#     cargo, 'build',
#     cargo_options,
#     '&&',
#     'cp', 'e-helper' / 'src' / rust_target / 'e-helper', '@OUTPUT@',
#   ]
# )

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
# cargo_options = [ '--target-dir', meson.project_build_root() / 'src' ]
# cargo_options += [ '--locked' ]

if get_option('profile') == 'default'
  cargo_options += [ '--release' ]
  rust_target = 'release'
  message('Building in release mode')
else
  rust_target = 'debug'
  message('Building in debug mode')
endif

# cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
cargo_env = [ 'CARGO_HOME=/build/source/.cargo-home' ]

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  depends: resources,
  command: [
    # 'env',
    # cargo_env,
    cargo, 'build',
    ['--release', '--manifest-path', meson.project_source_root() / 'Cargo.toml'],
    # cargo_options,
    # '&&',
    # 'ls', '-lah',
    # '&&',
    # 'env',
    # '&&',
    # 'exit',  '1',
    '&&',
    # 'cp', 'target' / rust_target / meson.project_name(), '@OUTPUT@',
    'cp', '/build/source/target/' / rust_target / meson.project_name(), '@OUTPUT@',


  ]
)
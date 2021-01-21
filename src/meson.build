conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('localedir', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('pkgdatadir', health_data_dir)
conf.set_quoted('application_id', application_id)
conf.set_quoted('log_domain', application_id)

configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.build_root(), 'src', 'config.rs'),
  join_paths(meson.source_root(), 'src', 'config.rs'),
  check: true
)

sources = files(
  'main.rs',
  'config.rs',
  'core/application.rs',
  'core/mod.rs',
  'core/settings.rs',
  'core/database.rs',
  'core/i18n.rs',
  'model/mod.rs',
  'model/activity.rs',
  'model/steps_graph_model.rs',
  'model/weight_graph_model.rs',
  'windows/window.rs',
  'windows/mod.rs',
  'views/graph_view.rs',
  'views/view.rs',
  'views/mod.rs',
)

cargo_script = find_program(join_paths(meson.source_root(), 'build-aux/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: 'meson.project_name()',
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.build_root(),
    meson.source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    'health',
  ]
)

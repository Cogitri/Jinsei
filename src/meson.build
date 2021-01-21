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
  'config.rs',
  'core/application.rs',
  'core/database.rs',
  'core/i18n.rs',
  'core/mod.rs',
  'core/settings.rs',
  'main.rs',
  'model/activity.rs',
  'model/mod.rs',
  'model/graph_model.rs',
  'model/graph_model_steps.rs',
  'model/graph_model_weight.rs',
  'views/graph_view.rs',
  'views/mod.rs',
  'views/view.rs',
  'views/view_steps.rs',
  'widgets/activity_row.rs',
  'widgets/mod.rs',
  'windows/mod.rs',
  'windows/window.rs',
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

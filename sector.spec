Name:           sector
Version:        0.1.0
Release:        1%{?dist}
Summary:        A GTK4/Rust application

License:        GPL-3.0-or-later
URL:            https://github.com/vani_tty1/sector
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  meson
BuildRequires:  ninja-build
BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  pkgconfig(gtk4)
BuildRequires:  pkgconfig(libadwaita-1)
BuildRequires:  blueprint-compiler
BuildRequires:  gettext
BuildRequires:  glib2-devel
BuildRequires:  desktop-file-utils

Requires:       gtk4
Requires:       libadwaita

%description
WIP

%prep
%autosetup

%build
%meson
%meson_build

%install
%meson_install
%find_lang %{name}

%check
desktop-file-validate %{buildroot}%{_datadir}/applications/io.github.vani_tty1.sector.desktop

%files -f %{name}.lang
%license COPYING
%doc README.md
%{_bindir}/sector
%{_datadir}/applications/io.github.vani_tty1.sector.desktop
%{_datadir}/glib-2.0/schemas/io.github.vani_tty1.sector.gschema.xml
%{_datadir}/icons/hicolor/scalable/apps/io.github.vani_tty1.sector.svg
%{_datadir}/icons/hicolor/symbolic/apps/io.github.vani_tty1.sector-symbolic.svg
%{_datadir}/metainfo/io.github.vani_tty1.sector.metainfo.xml
%{_datadir}/dbus-1/services/io.github.vani_tty1.sector.service
%{_datadir}/sector/
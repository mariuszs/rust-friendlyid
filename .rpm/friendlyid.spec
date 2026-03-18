%define debug_package %{nil}

Name: friendlyid
Summary: FriendlyID tool — converts UUIDs to URL-friendly Base62 IDs and vice versa
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT OR Apache-2.0
URL: https://github.com/mariuszs/rust-friendlyid
Source0: %{name}-%{version}.tar.gz

%description
%{summary}

%prep
%setup -q

%install
mkdir -p %{buildroot}
cp -a * %{buildroot}

%files
%{_bindir}/*

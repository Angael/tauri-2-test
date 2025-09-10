import {
  AppShell,
  Group,
  Burger,
  NavLink as MantineNavLink,
  Container,
  Stack
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { NavLink, useLocation } from "react-router";

type Props = {
  children?: React.ReactNode;
};

const Layout = ({ children }: Props) => {
  const [opened, { toggle }] = useDisclosure(false);
  const location = useLocation();

  const isStillViewer = location.pathname.startsWith("/viewer-dir");

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{
        width: 300,
        breakpoint: "sm",
        collapsed: { mobile: !opened, desktop: !opened }
      }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger opened={opened} onClick={toggle} size="sm" />
          <div style={{ fontWeight: "bold", fontSize: "1.2rem" }}>
            Camille 2
          </div>
        </Group>
      </AppShell.Header>
      <AppShell.Navbar p="md">
        <MantineNavLink
          component={NavLink}
          to="/"
          label="Viewer"
          style={{ textDecoration: "none" }}
          active={isStillViewer}
        />
        <MantineNavLink component={NavLink} to="/config" label="Config" />
        <MantineNavLink
          color="red"
          component={NavLink}
          to="/debug"
          label="Debug"
        />
      </AppShell.Navbar>
      <AppShell.Main>{children}</AppShell.Main>
    </AppShell>
  );
};

export default Layout;

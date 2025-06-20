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
  containerProps?: React.ComponentProps<typeof Container>;
};

const Layout = ({ children, containerProps }: Props) => {
  const [opened, { toggle }] = useDisclosure();
  const location = useLocation();

  const isStillViewer = location.pathname.startsWith("/viewer-dir");

  return (
    <AppShell
      header={{ height: 60 }}
      navbar={{ width: 300, breakpoint: "sm", collapsed: { mobile: !opened } }}
      padding="md"
    >
      <AppShell.Header>
        <Group h="100%" px="md">
          <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
          <div style={{ fontWeight: "bold", fontSize: "1.2rem" }}>My App</div>
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
        <MantineNavLink
          component={NavLink}
          to="/config"
          label="Config"
          // style={{ textDecoration: "none" }}
        />
      </AppShell.Navbar>
      <AppShell.Main>
        <Container {...containerProps}>
          <Stack>{children}</Stack>
        </Container>
      </AppShell.Main>
    </AppShell>
  );
};

export default Layout;

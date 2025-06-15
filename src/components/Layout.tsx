import {
  AppShell,
  Group,
  Burger,
  NavLink as MantineNavLink,
  Container,
  Stack
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { NavLink } from "react-router";

type Props = {
  children: React.ReactNode;
};

const Layout = ({ children }: Props) => {
  const [opened, { toggle }] = useDisclosure();

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
        />
        <MantineNavLink
          component={NavLink}
          to="/saved-dirs"
          label="Saved Dirs"
          style={{ textDecoration: "none" }}
        />
        <MantineNavLink
          component={NavLink}
          to="/config"
          label="Config"
          // style={{ textDecoration: "none" }}
        />
      </AppShell.Navbar>
      <AppShell.Main>
        <Container>
          <Stack>{children}</Stack>
        </Container>
      </AppShell.Main>
    </AppShell>
  );
};

export default Layout;

import {
  ActionIcon,
  AppShell,
  Burger,
  Divider,
  Group,
  NavLink as MantineNavLink,
  Paper,
  Stack,
  Text
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { mdiBugOutline, mdiCogOutline, mdiHomeOutline } from "@mdi/js";
import Icon from "@mdi/react";
import { NavLink, useLocation, useParams } from "react-router";
import { splitPathAndFilename } from "../util/splitPathAndFilename";

type Props = {
  children?: React.ReactNode;
};

const Layout = ({ children }: Props) => {
  const [opened, { toggle }] = useDisclosure(false);
  const location = useLocation();
  const params = useParams();

  const [dirPath, folderName] = params.dirPath
    ? splitPathAndFilename(params.dirPath)
    : ["", ""];

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
          <ActionIcon
            variant="subtle"
            color="gray"
            size="lg"
            component={NavLink}
            to="/"
          >
            <Icon path={mdiHomeOutline} size={1} />
          </ActionIcon>

          {dirPath && (
            <Paper withBorder p="xs" radius="md" bg="gray.0">
              <Stack gap="0">
                <Text size="xs" lh="0.7rem">
                  {dirPath}
                </Text>
                <Text fw={600} lh="1.2rem">
                  {folderName}
                </Text>
              </Stack>
            </Paper>
          )}

          <Divider orientation="vertical" ml="auto" />

          <ActionIcon
            variant="subtle"
            color="gray"
            size="lg"
            component={NavLink}
            to="/debug"
          >
            <Icon path={mdiBugOutline} size={1} />
          </ActionIcon>
          <ActionIcon
            variant="subtle"
            color="gray"
            size="lg"
            component={NavLink}
            to="/config"
          >
            <Icon path={mdiCogOutline} size={1} />
          </ActionIcon>
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

      <AppShell.Main style={{ overflowX: "clip" }}>{children}</AppShell.Main>
    </AppShell>
  );
};

export default Layout;

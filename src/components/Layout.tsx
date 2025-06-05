import { Group, Stack } from "@mantine/core";
import { NavLink } from "react-router";

type Props = {
  children: React.ReactNode;
} & React.ComponentProps<typeof Stack>;

const Layout = ({ children, ...props }: Props) => {
  return (
    <Stack {...props}>
      <Group>
        <NavLink to="/" style={{ textDecoration: "none" }}>
          Todos
        </NavLink>
        <NavLink to="/saved-dirs" style={{ textDecoration: "none" }}>
          Saved Dirs
        </NavLink>
      </Group>
      <main>{children}</main>
    </Stack>
  );
};

export default Layout;
